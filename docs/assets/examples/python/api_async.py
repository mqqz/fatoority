import asyncio
import os

from fatoora import AsyncZatcaClient, ClearanceMode, Config, Environment, ProductionCsid, SignedInvoice

# Requires httpx: pip install "fatoority[async]"
# Use credentials issued for this same environment.
environment = Environment.NON_PRODUCTION
token = os.environ["ZATCA_TOKEN"]
secret = os.environ["ZATCA_SECRET"]


async def main():
    with Config(environment) as config, \
            ProductionCsid(environment, token=token, secret=secret) as credentials, \
            SignedInvoice.from_file("signed.xml") as invoice:
        async with AsyncZatcaClient(config, timeout=30) as client:
            with await client.report_simplified_invoice(
                invoice, credentials, clearance=ClearanceMode.DISABLED, accept_language="en"
            ) as response:
                response.ensure_accepted()
                print("Invoice reporting accepted:", response.reporting_status)


asyncio.run(main())
