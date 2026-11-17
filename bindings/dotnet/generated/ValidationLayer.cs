namespace Fatoora;

public enum ValidationLayer : int
{
    Unknown = 0,
    FieldChecks = 1,
    Xsd = 2,
    BusinessRules = 3,
    Signature = 4,
    Qr = 5,
    PreviousInvoiceHash = 6,
}