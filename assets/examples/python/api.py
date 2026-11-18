import os
from fatoora import ClearanceMode, Config, Environment, ProductionCsid, SignedInvoice, ZatcaClient

# Use credentials issued for this same environment.
environment = Environment.NON_PRODUCTION
token = os.environ["ZATCA_TOKEN"]
secret = os.environ["ZATCA_SECRET"]
with Config(environment) as config, ZatcaClient(config) as client:
    with ProductionCsid(environment, token=token, secret=secret) as credentials:
        with SignedInvoice.from_file("signed.xml") as invoice:
            with client.report_simplified_invoice(
                invoice, credentials, clearance=ClearanceMode.DISABLED, accept_language="en"
            ) as response:
                response.ensure_accepted()
                print("Invoice reporting accepted:", response.reporting_status)
