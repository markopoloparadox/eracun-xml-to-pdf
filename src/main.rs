/*
	Referentni Dokument: https://www.fina.hr/ngsite/content/download/12522/187213/1
*/

use std::{fs::File, io::BufReader};
use xml::{EventReader, name::OwnedName, reader::XmlEvent};

#[derive(Default, Debug, serde::Serialize)]
pub struct XmlData {
	// Broj računa
	// ID: BT-1
	// /Invoice/cbc:ID
	// 1..1
	pub id: String,
	// Datum izdavanja računa
	// ID: BT-2
	// /Invoice/cbc:IssueDate
	// 1..1
	pub issue_date: String,
	// Šifra vrste računa
	// ID: BT-3
	// /Invoice/cbc:InvoiceTypeCode
	// 1..1
	pub invoice_type_code: String,
	// Šifra valute računa
	// ID: BT-5
	// /Invoice/cbc:DocumentCurrencyCode
	// 1..1
	pub document_currency_code: String,
	// Šifra valute obračunatog PDV-a
	// ID: BT-6
	// /Invoice/cbc:TaxCurrencyCode
	// 0..1
	pub tax_currency_code: Option<String>,
	// Datum na koji porez na dodanu vrijednost postaje važeći
	// ID: BT-7
	// /Invoice/cbc:TaxPointDate
	// 0..1
	pub tax_point_date: Option<String>,
	// Šifra datuma na koji porez na dodanu vrijednost postaje važeći
	// ID: BT-8
	// /Invoice/cbc:DescriptionCode
	// 0..1
	pub description_code: Option<String>,
	// Datum dospijeća plaćanja
	// ID: BT-9
	// /Invoice/cbc:DueDate
	// 0..1
	pub due_date: Option<String>,
	// Referenca na  kupca
	// ID: BT-10
	// /Invoice/cbc:BuyerReference
	// 0..1
	pub buyer_reference: Option<String>,
	// Mjesto troška
	// ID: BT-19
	// /Invoice/cbc:AccountingCost
	// 0..1
	pub accounting_cost: Option<String>,
	// Datum početka obračunskog razdoblja
	// ID: BT-73
	// /Invoice/cac:InvoicePeriod/cbc:StartDate
	// 0..1
	pub invoice_period_state_date: Option<String>,
	// Datum završetka obračunskog razdoblja
	// ID: BT-74
	// /Invoice/cac:InvoicePeriod/cbc:EndDate
	// 0..1
	pub invoice_period_end_date: Option<String>,
	// Notes
	// TODO
	pub notes: Vec<String>,
	// UKUPNI IZNOSI
	// ID: BG-22
	// /Invoice/cac:LegalMonetaryTotal
	// 1..1
	pub legal_monetary_total: LegalMonetaryTotal,
	// UPUTE ZA PLAĆANJE
	// ID: BG-16
	// /Invoice/cac:PaymentMeans
	// 0..1
	pub payment_means: Option<PaymentMeans>,
	// PRODAVATELJ
	// ID: BG-4
	// /Invoice/cac:AccountingSupplierParty
	// 1..1
	pub accounting_supplier_party: AccountingSupplierParty,
	// KUPAC
	// ID: BG-7
	// /Invoice/cac:AccountingCustomerParty
	// 1..1
	pub accounting_customer_party: AccountingCustomerParty,
	// STAVKA RAČUNA
	// ID: BG-25
	// /Invoice/cac:InvoiceLine
	// 1..n
	pub invoice_lines: Vec<InvoiceLine>,

	// Path stuff
	#[serde(skip)]
	pub current_path: Vec<String>,
	#[serde(skip)]
	pub depth: u32,
}

impl XmlData {
	pub fn push_path(&mut self, name: &OwnedName) {
		self.depth += 1;
		self.current_path.push(to_prefixed_name(name));
	}

	pub fn pop_path(&mut self) {
		self.depth += 1;
		self.current_path.pop();
	}

	pub fn path(&self) -> String {
		let mut path = String::new();
		for p in self.current_path.iter() {
			path = std::format!("{}/{}", path, p);
		}

		path
	}

	pub fn new_invoice_line(&mut self, id: String) {
		let mut line = InvoiceLine::default();
		line.id = id;
		self.invoice_lines.push(line);
	}

	pub fn invoice_line(&mut self) -> &mut InvoiceLine {
		self.invoice_lines.last_mut().unwrap()
	}

	pub fn new_payee_financial_account(&mut self, id: String) {
		let mut a = PayeeFinancialAccount::default();
		a.id = id;
		self.payment_means().payee_financial_accounts.push(a);
	}

	pub fn payment_means(&mut self) -> &mut PaymentMeans {
		if self.payment_means.is_none() {
			self.payment_means = Some(PaymentMeans::default());
		}

		self.payment_means.as_mut().unwrap()
	}
}

#[derive(Debug, Default, serde::Serialize)]
pub struct AccountingCustomerParty {}

#[derive(Debug, Default, serde::Serialize)]
pub struct PostalAddress {
	// Redak adrese Prodavatelja 1
	// ID: BT-35
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cbc:StreetName
	// 0..1
	street_name: Option<String>,
	// Redak adrese Prodavatelja 2
	// ID: BT-36
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cbc:AdditionalStreetName
	// 0..1
	additional_street_name: Option<String>,
	// Redak adrese Prodavatelja 3
	// ID: BT-162
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cac:AddressLine/cbc:Line
	// 0..1
	line: Option<String>,
	// Grad Prodavatelja
	// ID: BT-37
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cbc:CityName
	// 0..1
	city_name: Option<String>,
	// Poštanski broj Prodavatelja
	// ID: BT-38
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cbc:PostalZone
	// 0..1
	postal_zone: Option<String>,
	// Županija Prodavatelja
	// ID: BT-39
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cbc:CountrySubentity
	// 0..1
	country_subentity: Option<String>,
	// Šifra države Prodavatelja
	// ID: BT-40
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cac:Country/cbc:IdentificationCode
	// 1..1
	identification_code: String,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct AccountingSupplierParty {
	// Naziv Prodavatelja
	// ID: BT-27
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cac:PartyLegalEntity/cbc:RegistrationName
	// 1..1
	registration_name: String,
	// Dodatne pravne informacije o Prodavatelju
	// ID: BT-33
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cac:PartyLegalEntity/cbc:CompanyLegalForm
	// 1..1
	company_legal_form: String,
	// Elektronička adresa Prodavatelja
	// ID: BT-34
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cbc:EndpointID
	// 1..1
	endpoint_id: String,
	// Identifikator sheme
	// ID: BT-34
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cbc:EndpointID/@schemeID
	// 1..1
	endpoint_id_scheme_id: String,
	// POŠTANSKA ADRESA PRODAVATELJA
	// ID: BG-5
	// /Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress
	// 1..1
	postal_address: PostalAddress,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct PayeeFinancialAccount {
	// Identifikator računa plaćanja
	// ID: BT-84
	// /Invoice/cac:PaymentMeans/cac:PayeeFinancialAccount/cbc:ID
	// 1..1
	id: String,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct PaymentMeans {
	// Šifra načina plaćanja
	// ID: BT-81
	// /Invoice/cac:PaymentMeans/cbc:PaymentMeansCode
	// 1..1
	payment_means_code: String,
	// Tekst načina plaćanja
	// ID: BT-82
	// /Invoice/cac:PaymentMeans/cbc:InstructionNote
	// 0..1
	instruction_note: Option<String>,
	// Informacije o doznaci
	// ID: BT-83
	// /Invoice/cac:PaymentMeans/cbc:PaymentID
	// 0..1
	payment_id: Option<String>,
	// KREDITNI TRANSFER
	// ID: BG-17
	// /Invoice/cac:PaymentMeans/cac:PayeeFinancialAccount
	// 0..n
	payee_financial_accounts: Vec<PayeeFinancialAccount>,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct LegalMonetaryTotal {
	// Zbroj svih neto iznosa stavki računa
	// ID: BT-106
	// /Invoice/cac:LegalMonetaryTotal/cbc:LineExtensionAmount
	// 1..1
	line_extension_amount: String,
	// ???
	// TODO
	line_extension_amount_currency_id: Option<String>,
	// Ukupni iznos računa bez PDV-a
	// ID: BT-109
	// /Invoice/cac:LegalMonetaryTotal/cbc:TaxExclusiveAmount
	// 1..1
	tax_exclusive_amount: String,
	// ???
	// TODO
	tax_exclusive_amount_currency_id: Option<String>,
	// Ukupni iznos računa s PDV-om
	// ID: BT-112
	// /Invoice/cac:LegalMonetaryTotal/cbc:TaxInclusiveAmount
	// 1..1
	tax_inclusive_amount: String,
	// ???
	// TODO
	tax_inclusive_amount_currency_id: Option<String>,
	// Iznos koji dospijeva na plaćanje
	// ID: BT-115
	// /Invoice/cac:LegalMonetaryTotal/cbc:PayableAmount
	// 1..1
	payable_amount: String,
	// ???
	// TODO
	payable_amount_currency_id: Option<String>,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct InvoiceLine {
	// Identifikator stavke računa
	// ID: BT-126
	// /Invoice/cac:InvoiceLine/cbc:ID
	// 1..1
	id: String,
	// Obračunata količina
	// ID: BT-129
	// /Invoice/cac:InvoiceLine/cbc:InvoicedQuantity
	// 1..1
	invoiced_quantity: String,
	// Šifra jedinica mjere obračunate količine
	// ID: BT-130
	// /Invoice/cac:InvoiceLine/cbc:InvoicedQuantity/@unitCode
	// 1..1
	invoiced_quantity_unit_code: String,
	// Neto iznos stavke računa
	// ID: BT-131
	// /Invoice/cac:InvoiceLine/cbc:LineExtensionAmount
	// 1..1
	line_extension_amount: String,
	// DETALJI O CIJENI
	// ID: BG-29
	// /Invoice/cac:InvoiceLine/cac:Price
	// 1..1
	price: Price,
	// INFORMACIJE O ARTIKLU
	// ID: BG-31
	// /Invoice/cac:InvoiceLine/cac:Item
	// 1..1
	item: Item,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct Price {
	// Neto cijena artikla
	// ID: BT-146
	// /Invoice/cac:InvoiceLine/cac:Price/cbc:PriceAmount
	// 1..1
	price_amount: String,
	// Jedinična količina cijene artikla
	// ID: BT-149
	// /Invoice/cac:InvoiceLine/cac:Price/cbc:BaseQuantity
	// 0..1
	base_quantity: Option<String>,
	// Šifra jedinice mjere jedinične količine artikla
	// ID: BT-150
	// /Invoice/cac:InvoiceLine/cac:Price/cbc:BaseQuantity/@unitCode
	// 0..1
	base_quantity_unit_code: Option<String>,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct Item {
	// Naziv artikla
	// ID: BT-153
	// /Invoice/cac:InvoiceLine/cac:Item/cbc:Name
	// 1..1
	name: String,
	// Opis artikla
	// ID: BT-154
	// /Invoice/cac:InvoiceLine/cac:Item/cbc:Description
	// 0..1
	description: Option<String>,
	// INFORMACIJA O PDV-u STAVKE RAČUNA
	// ID: BG-30
	// /Invoice/cac:InvoiceLine/cac:Item/cac:ClassifiedTaxCategory
	// 1..1
	classified_tax_category: ClassifiedTaxCategory,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct ClassifiedTaxCategory {
	// Šifra kategorije PDV-a obračunate stavke
	// ID: BT-151
	// /Invoice/cac:InvoiceLine/cac:Item/cac:ClassifiedTaxCategory/cbc:ID
	// 1..1
	id: String,
	// Stopa PDV-a obračunate stavke
	// ID: BT-152
	// /Invoice/cac:InvoiceLine/cac:Item/cac:ClassifiedTaxCategory/cbc:Percent
	// 0..1
	percent: Option<String>,
	// ???
	// /Invoice/cac:InvoiceLine/cac:Item/cac:ClassifiedTaxCategory/cac:TaxScheme/cbc:ID
	tax_scheme_id: Option<String>,
}

fn main() {
	let file = File::open("./example.xml").unwrap();

	let buffer = BufReader::new(file);
	let mut parser = EventReader::new(buffer);
	let mut xml_data = XmlData::default();
	skip_ubl_extensions(&mut parser, &mut xml_data);
	main_logic(&mut parser, &mut xml_data);

	dbg!(&xml_data);
	std::fs::write("./file.json", serde_json::to_string_pretty(&xml_data).unwrap());
}

pub fn main_logic(parser: &mut EventReader<BufReader<File>>, xml_data: &mut XmlData) {
	while let Ok(e) = parser.next() {
		if matches!(e, XmlEvent::EndDocument) {
			return;
		}

		if matches!(e, XmlEvent::EndElement { .. }) {
			xml_data.pop_path();
			continue;
		}

		let XmlEvent::StartElement { name, attributes, .. } = e else {
			continue;
		};
		xml_data.push_path(&name);

		let path = xml_data.path();
		dbg!(xml_data.path());
		match path.as_str() {
			"/Invoice/cbc:ID" => xml_data.id = read_string(parser),
			"/Invoice/cbc:IssueDate" => xml_data.issue_date = read_string(parser),
			"/Invoice/cbc:InvoiceTypeCode" => xml_data.invoice_type_code = read_string(parser),
			"/Invoice/cbc:DocumentCurrencyCode" => xml_data.document_currency_code = read_string(parser),
			"/Invoice/cbc:TaxCurrencyCode" => xml_data.tax_currency_code = Some(read_string(parser)),
			"/Invoice/cbc:TaxPointDate" => xml_data.tax_point_date = Some(read_string(parser)),
			"/Invoice/cbc:DescriptionCode" => xml_data.description_code = Some(read_string(parser)),
			"/Invoice/cbc:DueDate" => xml_data.due_date = Some(read_string(parser)),
			"/Invoice/cbc:BuyerReference" => xml_data.buyer_reference = Some(read_string(parser)),
			"/Invoice/cbc:Note" => xml_data.notes.push(read_string(parser)),
			"/Invoice/cbc:AccountingCost" => xml_data.accounting_cost = Some(read_string(parser)),
			"/Invoice/cac:InvoicePeriod/cbc:StartDate" => {
				xml_data.invoice_period_state_date = Some(read_string(parser))
			},
			"/Invoice/cac:InvoicePeriod/cbc:EndDate" => xml_data.invoice_period_end_date = Some(read_string(parser)),
			"/Invoice/cac:InvoiceLine/cbc:ID" => xml_data.new_invoice_line(read_string(parser)),
			"/Invoice/cac:InvoiceLine/cbc:InvoicedQuantity" => {
				xml_data.invoice_line().invoiced_quantity = read_string(parser);
				let unit_code = attributes.iter().find(|x| x.name.local_name == "unitCode").unwrap();
				xml_data.invoice_line().invoiced_quantity_unit_code = unit_code.value.clone();
			},
			"/Invoice/cac:InvoiceLine/cbc:LineExtensionAmount" => {
				xml_data.invoice_line().line_extension_amount = read_string(parser)
			},
			"/Invoice/cac:InvoiceLine/cac:Price/cbc:PriceAmount" => {
				xml_data.invoice_line().price.price_amount = read_string(parser)
			},
			"/Invoice/cac:InvoiceLine/cac:Price/cbc:BaseQuantity" => {
				xml_data.invoice_line().price.base_quantity = Some(read_string(parser));
				if let Some(unit_code) = attributes.iter().find(|x| x.name.local_name == "unitCode") {
					xml_data.invoice_line().price.base_quantity_unit_code = Some(unit_code.value.clone());
				}
			},
			"/Invoice/cac:InvoiceLine/cac:Item/cac:ClassifiedTaxCategory/cbc:ID" => {
				xml_data.invoice_line().item.classified_tax_category.id = read_string(parser);
			},
			"/Invoice/cac:InvoiceLine/cac:Item/cac:ClassifiedTaxCategory/cbc:Percent" => {
				xml_data.invoice_line().item.classified_tax_category.percent = Some(read_string(parser));
			},
			"/Invoice/cac:InvoiceLine/cac:Item/cac:ClassifiedTaxCategory/cac:TaxScheme/cbc:ID" => {
				xml_data.invoice_line().item.classified_tax_category.tax_scheme_id = Some(read_string(parser));
			},
			"/Invoice/cac:InvoiceLine/cac:Item/cbc:Name" => {
				xml_data.invoice_line().item.name = read_string(parser);
			},
			"/Invoice/cac:InvoiceLine/cac:Item/cbc:Description" => {
				xml_data.invoice_line().item.description = Some(read_string(parser));
			},
			"/Invoice/cac:LegalMonetaryTotal/cbc:LineExtensionAmount" => {
				xml_data.legal_monetary_total.line_extension_amount = read_string(parser);
				if let Some(currency_id) = attributes.iter().find(|x| x.name.local_name == "currencyID") {
					xml_data.legal_monetary_total.line_extension_amount_currency_id = Some(currency_id.value.clone());
				}
			},
			"/Invoice/cac:LegalMonetaryTotal/cbc:TaxExclusiveAmount" => {
				xml_data.legal_monetary_total.tax_exclusive_amount = read_string(parser);
				if let Some(currency_id) = attributes.iter().find(|x| x.name.local_name == "currencyID") {
					xml_data.legal_monetary_total.tax_exclusive_amount_currency_id = Some(currency_id.value.clone());
				}
			},
			"/Invoice/cac:LegalMonetaryTotal/cbc:TaxInclusiveAmount" => {
				xml_data.legal_monetary_total.tax_inclusive_amount = read_string(parser);
				if let Some(currency_id) = attributes.iter().find(|x| x.name.local_name == "currencyID") {
					xml_data.legal_monetary_total.tax_inclusive_amount_currency_id = Some(currency_id.value.clone());
				}
			},
			"/Invoice/cac:LegalMonetaryTotal/cbc:PayableAmount" => {
				xml_data.legal_monetary_total.payable_amount = read_string(parser);
				if let Some(currency_id) = attributes.iter().find(|x| x.name.local_name == "currencyID") {
					xml_data.legal_monetary_total.payable_amount_currency_id = Some(currency_id.value.clone());
				}
			},
			"/Invoice/cac:PaymentMeans/cbc:PaymentMeansCode" => {
				xml_data.payment_means().payment_means_code = read_string(parser);
			},
			"/Invoice/cac:PaymentMeans/cbc:InstructionNote" => {
				xml_data.payment_means().instruction_note = Some(read_string(parser));
			},
			"/Invoice/cac:PaymentMeans/cbc:PaymentID" => {
				xml_data.payment_means().payment_id = Some(read_string(parser));
			},
			"/Invoice/cac:PaymentMeans/cac:PayeeFinancialAccount/cbc:ID" => {
				xml_data.new_payee_financial_account(read_string(parser));
			},
			"/Invoice/cac:AccountingSupplierParty/cac:Party/cac:PartyLegalEntity/cbc:RegistrationName" => {
				xml_data.accounting_supplier_party.registration_name = read_string(parser);
			},
			"/Invoice/cac:AccountingSupplierParty/cac:Party/cac:PartyLegalEntity/cbc:CompanyLegalForm" => {
				xml_data.accounting_supplier_party.company_legal_form = read_string(parser);
			},
			"/Invoice/cac:AccountingSupplierParty/cac:Party/cbc:EndpointID" => {
				xml_data.accounting_supplier_party.endpoint_id = read_string(parser);
				let scheme_id = attributes.iter().find(|x| x.name.local_name == "schemeID").unwrap();
				xml_data.accounting_supplier_party.endpoint_id_scheme_id = scheme_id.value.clone();
			},
			"/Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cbc:StreetName" => {
				xml_data.accounting_supplier_party.postal_address.street_name = Some(read_string(parser));
			},
			"/Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cbc:AdditionalStreetName" => {
				xml_data.accounting_supplier_party.postal_address.additional_street_name = Some(read_string(parser));
			},
			"/Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cac:AddressLine/cbc:Line" => {
				xml_data.accounting_supplier_party.postal_address.line = Some(read_string(parser));
			},
			"/Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cbc:CityName" => {
				xml_data.accounting_supplier_party.postal_address.city_name = Some(read_string(parser));
			},
			"/Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cbc:PostalZone" => {
				xml_data.accounting_supplier_party.postal_address.postal_zone = Some(read_string(parser));
			},
			"/Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cbc:CountrySubentity" => {
				xml_data.accounting_supplier_party.postal_address.country_subentity = Some(read_string(parser));
			},
			"/Invoice/cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cac:Country/cbc:IdentificationCode" => {
				xml_data.accounting_supplier_party.postal_address.identification_code = read_string(parser);
			},
			_ => (),
		}
	}
}

pub fn read_string(parser: &mut EventReader<BufReader<File>>) -> String {
	let e = parser.next().unwrap();
	if let XmlEvent::Characters(chars) = e {
		return chars.trim().to_owned();
	};

	panic!("Not Chars :(")
}

pub fn skip_ubl_extensions(parser: &mut EventReader<BufReader<File>>, xml_data: &mut XmlData) {
	move_to_element_w_prefix(parser, "ext:UBLExtensions", false, xml_data).unwrap();
	move_to_element_w_prefix(parser, "ext:UBLExtensions", true, xml_data).unwrap();
}

pub fn move_to_element_w_prefix(
	parser: &mut EventReader<BufReader<File>>,
	elem_name: &str,
	end: bool,
	xml_data: &mut XmlData,
) -> Result<(), ()> {
	while let Ok(e) = parser.next() {
		match e {
			XmlEvent::StartElement { name, .. } => {
				xml_data.push_path(&name);
				if !end {
					if let Some(prefix) = &name.prefix {
						if std::format!("{}:{}", prefix, name.local_name).as_str() == elem_name {
							println!("Start: {}", &name.local_name);
							return Ok(());
						}
					}
				}
			},
			XmlEvent::EndElement { name, .. } => {
				xml_data.pop_path();
				if end {
					if let Some(prefix) = &name.prefix {
						if std::format!("{}:{}", prefix, name.local_name).as_str() == elem_name {
							println!("End: {}", &name.local_name);
							return Ok(());
						}
					}
				}
			},
			XmlEvent::EndDocument => {
				panic!("End");
			},
			_ => (),
		}
	}

	Err(())
}

fn to_prefixed_name(name: &OwnedName) -> String {
	if let Some(prefix) = &name.prefix {
		std::format!("{}:{}", prefix, name.local_name)
	} else {
		name.local_name.clone()
	}
}
