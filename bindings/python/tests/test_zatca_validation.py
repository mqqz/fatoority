import json
from dataclasses import FrozenInstanceError
from datetime import datetime, timedelta, timezone
from pathlib import Path

import pytest

from fatoora import (
    InvalidInputError,
    Severity,
    ValidationError,
    ValidationLayer,
    ValidationLocationKind,
    XmlError,
    ZatcaReport,
    ZatcaStage,
    ZatcaStageStatus,
    validate_xml,
    validate_zatca,
)

CORPUS = (
    Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures/sdk-parity/cases"
)
SEED = "NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ=="
OPTIONS = {"previous_invoice_hash": SEED, "evaluated_at": "2026-09-23T12:00:00+03:00"}


def test_valid_warning_and_rejected_reports_preserve_source_findings():
    for kind in ["standard", "simplified"]:
        xml = (CORPUS / f"{kind}-invoice/sdk-signed.xml").read_text()
        report = validate_zatca(xml, **OPTIONS)
        assert isinstance(report, ZatcaReport)
        assert report.is_valid and report.is_complete and not report.has_errors
        assert [s.stage for s in report.stages] == [
            ZatcaStage.XSD, ZatcaStage.CEN, ZatcaStage.KSA, ZatcaStage.SIGNATURE,
            ZatcaStage.QR, ZatcaStage.PREVIOUS_INVOICE_HASH]
        assert len(report.stage(ZatcaStage.CEN).evaluated_assertions) == 105
        ksa = report.stage(ZatcaStage.KSA)
        assert len(ksa.evaluated_assertions) == 152
        assert ksa.provenance.stylesheet.endswith(".xsl") and len(ksa.provenance.sha256) == 64
        warning = next(f for f in ksa.findings if f.severity is Severity.WARNING)
        assert warning.layer is ValidationLayer.BUSINESS_RULES
        assert warning.location.kind is ValidationLocationKind.X_PATH
        assert warning.location.path.startswith("/")
        assert warning.assertion_site.startswith("ksa:")
        assert report.warnings and not report.errors
        assert validate_xml(xml)
    rejected = validate_zatca("<wrong/>", **OPTIONS)
    assert rejected.has_errors and not rejected.is_valid
    finding = rejected.stages[0].findings[0]
    assert (finding.code, finding.severity, finding.layer) == ("XSD_INVALID", Severity.ERROR, ValidationLayer.XSD)
    assert finding.location is None
    assert all(s.status is ZatcaStageStatus.NOT_RUN for s in rejected.stages[1:])
    assert rejected.errors == (finding,)
    assert report.profile == "zatca-sdk-238-R3.4.8"
    assert report.schema_version == 1
    assert report.evaluated_at == datetime(2026, 9, 23, 9, tzinfo=timezone.utc)


def test_report_is_frozen_and_serialises_like_the_core_schema():
    report = validate_zatca("<wrong/>", **OPTIONS)
    with pytest.raises(FrozenInstanceError):
        report.is_valid = True
    data = report.to_dict()
    assert data == json.loads(report.to_json())
    assert data["stages"][0]["findings"][0]["code"] == "XSD_INVALID"
    assert data["is_valid"] is report.is_valid
    # String-valued enums compare equal to the JSON wire names.
    assert report.stages[0].stage == "xsd" and report.stages[0].findings[0].severity == "error"
    assert report == validate_zatca("<wrong/>", **OPTIONS)


def test_evaluated_at_accepts_aware_datetimes_only():
    when = datetime(2026, 9, 23, 12, tzinfo=timezone(timedelta(hours=3)))
    report = validate_zatca("<wrong/>", evaluated_at=when)
    assert report.evaluated_at == when
    with pytest.raises(ValueError, match="timezone-aware"):
        validate_zatca("<wrong/>", evaluated_at=datetime(2026, 9, 23, 12))


def test_missing_context_and_chain_mismatch():
    xml = (CORPUS / "standard-invoice/input.xml").read_text()
    missing = validate_zatca(xml)
    assert not missing.is_valid and not missing.is_complete
    assert missing.stage(ZatcaStage.PREVIOUS_INVOICE_HASH).status is ZatcaStageStatus.CONTEXT_REQUIRED
    wrong = validate_zatca(xml, previous_invoice_hash="A" * 43 + "=")
    assert wrong.is_complete and wrong.has_errors
    assert wrong.stage(ZatcaStage.PREVIOUS_INVOICE_HASH).findings[0].code == "PIH_MISMATCH"


def test_execution_failure_keeps_partial_report():
    xml = (CORPUS / "standard-invoice/input.xml").read_text()
    xml = xml.replace(
        '<cbc:LineExtensionAmount currencyID="SAR">300.00',
        '<cbc:LineExtensionAmount currencyID="[">300.00',
        1,
    )
    with pytest.raises(ValidationError) as caught:
        validate_zatca(xml, **OPTIONS)
    details = caught.value.details
    assert details["type"] == "zatca_validation_execution"
    assert details["assertion_site"] == "ksa:112:BR-KSA-CL-02"
    assert details["report"]["stages"][1]["status"] == "completed"
    assert details["report"]["stages"][2]["status"] == "evaluation_failed"
    assert not details["report"]["is_valid"]


def test_invalid_inputs_cannot_silently_truncate_or_succeed():
    for xml in ["<Invoice", "<Invoice>"]:
        with pytest.raises(XmlError) as caught:
            validate_zatca(xml, **OPTIONS)
        assert not caught.value.details["report"]["is_valid"]
    with pytest.raises(InvalidInputError):
        validate_zatca("<!DOCTYPE x><x/>", **OPTIONS)
    with pytest.raises(InvalidInputError):
        validate_zatca("<x/>", evaluated_at="bad")
    for kwargs in [
        {"xml": "<x/>\0ignored"},
        {"xml": "<x/>", "previous_invoice_hash": "\0"},
        {"xml": "<x/>", "evaluated_at": "\0"},
    ]:
        with pytest.raises(ValueError, match="interior NUL"):
            validate_zatca(**kwargs)
    with pytest.raises(TypeError):
        validate_zatca(b"<x/>")


def test_generated_report_rejects_unknown_and_oversized_options():
    from fatoora import _native

    for options in ('{"unknown":true}', ' ' * 4097):
        with pytest.raises(Exception) as caught:
            _native.Xml.validate_zatca("<x/>", options)
        assert caught.value.args[0].code() == 1


def test_generated_report_exposes_typed_stages():
    from fatoora import _native

    report = _native.Xml.validate_zatca("<wrong/>", '{"evaluated_at":"2026-09-23T12:00:00+03:00"}')
    assert not report.is_valid() and report.has_errors() and not report.is_complete()
    assert report.evaluated_at() == "2026-09-23T12:00:00+03:00"
    assert report.stages_len() == 6
    stage = report.stage(0)
    assert stage.stage() == _native.ZatcaStage.Xsd
    assert stage.status() == _native.ZatcaStageStatus.Completed
    finding = stage.finding(0)
    assert finding.code() == "XSD_INVALID"
    assert finding.severity() == _native.Severity.Error
    assert json.loads(report.to_json())["stages"][0]["findings"][0]["code"] == "XSD_INVALID"
