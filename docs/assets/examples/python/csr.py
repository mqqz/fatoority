from pathlib import Path
from fatoora import CsrProperties, Environment, SigningKey

with CsrProperties.from_file("csr.properties") as properties:
    with SigningKey.generate() as key:
        with properties.build(key, Environment.NON_PRODUCTION) as csr:
            Path("csr.pem").write_text(csr.to_pem(), encoding="utf-8")
            Path("key.pem").write_text(key.to_pem(), encoding="utf-8")
print("Saved csr.pem and key.pem")
