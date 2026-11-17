import os
from fatoora import Config, Environment, ProductionCsid, SignedInvoice, ZatcaClient

# Use credentials issued for this same environment.
environment = Environment.NON_PRODUCTION
token = os.environ["ZATCA_TOKEN"]
secret = os.environ["ZATCA_SECRET"]
with Config(environment) as config, ZatcaClient(config) as client:
    with ProductionCsid.new(environment, token, secret) as credentials:
        with SignedInvoice.from_file("signed.xml") as invoice:
            with client.report_simplified_invoice(
                invoice, credentials, clearance_status=False, accept_language="en"
            ) as response:
                response.ensure_accepted()
                print("Invoice reporting accepted:", response.reporting_status)
