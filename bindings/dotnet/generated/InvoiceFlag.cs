namespace Fatoora;

public enum InvoiceFlag : int
{
    ThirdParty = 1,
    Nominal = 2,
    Export = 4,
    Summary = 8,
    SelfBilled = 16,
}