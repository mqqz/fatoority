"""Constructor and declaration coverage for the public C facade."""
import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("generate_c_facade", Path(__file__).resolve().parents[1] / "generate_c_facade.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class FacadeDeclarations(unittest.TestCase):
    def generate(self, declaration):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            headers = root / "bindings/c/generated"
            headers.mkdir(parents=True)
            (root / "scripts").mkdir()
            (root / "scripts/c_facade_runtime.c").write_text("")
            (headers / "Example.h").write_text("typedef struct Example Example;\n" + declaration + "\n")
            with patch.object(MODULE, "ROOT", root):
                return MODULE.generate()

    def test_infallible_pointer_constructor_is_exposed(self):
        files, count = self.generate("Example* fatoora_Example_new(void);")
        self.assertEqual(count, 1)
        self.assertIn("fatoora_example_new(FatooraExample **out", files["fatoora.h"])
        self.assertIn("fatoora_Example_new()", files["fatoora.c"])

    def test_unrecognized_declarations_fail_instead_of_disappearing(self):
        with self.assertRaisesRegex(ValueError, "Unsupported function declaration"):
            self.generate("const Example* fatoora_Example_new(void);")


if __name__ == "__main__":
    unittest.main()
