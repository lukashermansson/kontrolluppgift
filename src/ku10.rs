use std::borrow::Cow;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{NsReader, Writer};
use crate::DeError::{MissingField, UnexpectedElement};
use crate::{DeError, to_bool, Write};

/// Kontrolluppgift 10
#[derive(Debug, PartialEq)]
pub struct KU10Type<'a> {
    pub kontant_bruttolon_mm: Option<i32>,
    pub forman_utom_bil_drivmedel: Option<i32>,
    pub bilforman_utom_drivmedel: Option<i32>,
    pub drivmedel_vid_bilforman: Option<i32>,
    pub andra_kostnadsers: Option<i32>,
    pub underlag_rutarbete: Option<i32>,
    pub underlag_rotarbete: Option<i32>,
    pub ers_m_egenavgifter: Option<i32>,
    pub tjanstepension: Option<i32>,
    pub ers_ej_soc_avg: Option<i32>,
    pub ers_ej_soc_avg_ej_jobbavd: Option<i32>,
    pub forsarskattenamnden: Option<i32>,
    pub vissa_avdrag: Option<i32>,
    pub hyresersattning: Option<i32>,
    pub bostad_smahus: Option<bool>,
    pub bostad_ej_smahus: Option<bool>,
    pub forman_har_justerats: Option<bool>,
    pub forman_som_pension: Option<bool>,
    pub bilersattning: Option<bool>,
    pub traktamente: Option<bool>,
    pub personaloption_forvarv_andel: Option<bool>,
    pub arbetsstallenummer: Option<Cow<'a, str>>,
    pub delagare: Option<bool>,
    pub social_avgifts_avtal: Option<bool>,
    pub inkomstar: Cow<'a, str>,
    pub borttag: Option<bool>,
    pub specifikationsnummer: i32,
    pub inkomsttagare: InkomsttagareKU10<'a>,
    pub uppgiftslamnare: UppgiftslamnareKU10<'a>,
}

impl<'a> KU10Type<'a> {
    pub(crate) fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("KU10").write_inner_content(|w| {
            w.write_node_with_code("KontantBruttolonMm", "011", &self.kontant_bruttolon_mm)?;
            w.write_node_with_code("FormanUtomBilDrivmedel", "012", &self.forman_utom_bil_drivmedel)?;
            w.write_node_with_code("BilformanUtomDrivmedel", "013", &self.bilforman_utom_drivmedel)?;
            w.write_node_with_code("DrivmedelVidBilforman", "018", &self.drivmedel_vid_bilforman)?;
            w.write_node_with_code("AndraKostnadsers", "020", &self.andra_kostnadsers)?;
            w.write_node_with_code("UnderlagRutarbete", "021", &self.underlag_rutarbete)?;
            w.write_node_with_code("UnderlagRotarbete", "022", &self.underlag_rotarbete)?;
            w.write_node_with_code("ErsMEgenavgifter", "025", &self.ers_m_egenavgifter)?;
            w.write_node_with_code("Tjanstepension", "030", &self.tjanstepension)?;
            w.write_node_with_code("ErsEjSocAvg", "031", &self.ers_ej_soc_avg)?;
            w.write_node_with_code("ErsEjSocAvgEjJobbavd", "032", &self.ers_ej_soc_avg_ej_jobbavd)?;
            w.write_node_with_code("Forskarskattenamnden", "035", &self.forsarskattenamnden)?;
            w.write_node_with_code("VissaAvdrag", "037", &self.vissa_avdrag)?;
            w.write_node_with_code("Hyresersattning", "039", &self.hyresersattning)?;
            w.write_node_with_code("BostadSmahus", "041", &self.bostad_smahus)?;
            w.write_node_with_code("BostadEjSmahus", "043", &self.bostad_ej_smahus)?;
            w.write_node_with_code("FormanHarJusterats", "048", &self.forman_har_justerats)?;
            w.write_node_with_code("FormanSomPension", "049", &self.forman_som_pension)?;
            w.write_node_with_code("Bilersattning", "050", &self.bilersattning)?;
            w.write_node_with_code("Traktamente", "051", &self.traktamente)?;
            w.write_node_with_code("PersonaloptionForvarvAndel", "059", &self.personaloption_forvarv_andel)?;
            w.write_node_with_code("Arbetsstallenummer", "060", &self.arbetsstallenummer)?;
            w.write_node_with_code("Delagare", "061", &self.delagare)?;
            w.write_node_with_code("SocialAvgiftsAvtal", "093", &self.social_avgifts_avtal)?;
            w.write_node_with_code("Inkomstar", "203", &self.inkomstar)?;
            w.write_node_with_code("Borttag", "205", &self.borttag)?;
            w.write_node_with_code("Specifikationsnummer", "570", &self.specifikationsnummer)?;

            self.inkomsttagare.write(w)?;
            self.uppgiftslamnare.write(w)?;
            Ok(())
        })?;
        Ok(())
    }
}

impl<'a> KU10Type<'a> {
    pub(crate) fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {
        let mut kontant_bruttolon_mm = None;
        let mut forman_utom_bil_drivmedel = None;
        let mut bilforman_utom_drivmedel = None;
        let mut drivmedel_vid_bilforman = None;
        let mut andra_kostnadsers = None;
        let mut underlag_rutarbete = None;
        let mut underlag_rotarbete = None;
        let mut ers_m_egenavgifter = None;
        let mut tjanstepension = None;
        let mut ers_ej_soc_avg = None;
        let mut ers_ej_soc_avg_ej_jobbavd = None;
        let mut forsarskattenamnden = None;
        let mut vissa_avdrag = None;
        let mut hyresersattning = None;
        let mut bostad_smahus = None;
        let mut bostad_ej_smahus = None;
        let mut forman_har_justerats = None;
        let mut forman_som_pension = None;
        let mut bilersattning = None;
        let mut traktamente = None;
        let mut personaloption_forvarv_andel = None;
        let mut arbetsstallenummer = None;
        let mut delagare = None;
        let mut social_avgifts_avtal = None;
        let mut inkomstar = None;
        let mut borttag = None;
        let mut specificationsnummer = None;
        let mut inkomsttagare = None;
        let mut uppgiftslamnare = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"KontantBruttolonMm" => {
                        kontant_bruttolon_mm = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"FormanUtomBilDrivmedel" => {
                        forman_utom_bil_drivmedel = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"BilformanUtomDrivmedel" => {
                        bilforman_utom_drivmedel = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"DrivmedelVidBilforman" => {
                        drivmedel_vid_bilforman = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"AndraKostnadsers" => {
                        andra_kostnadsers = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"UnderlagRutarbete" => {
                        underlag_rutarbete = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"UnderlagRotarbete" => {
                        underlag_rotarbete = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"ErsMEgenavgifter" => {
                        ers_m_egenavgifter = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"Tjanstepension" => {
                        tjanstepension = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"ErsEjSocAvg" => {
                        ers_ej_soc_avg = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"ErsEjSocAvgEjJobbavd" => {
                        ers_ej_soc_avg_ej_jobbavd = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"Forskarskattenamnden" => {
                        forsarskattenamnden = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"VissaAvdrag" => {
                        vissa_avdrag = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"Hyresersattning" => {
                        hyresersattning = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }
                    b"BostadSmahus" => {
                        bostad_smahus = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"BostadEjSmahus" => {
                        bostad_ej_smahus = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"FormanHarJusterats" => {
                        forman_har_justerats = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"FormanSomPension" => {
                        forman_som_pension = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"Bilersattning" => {
                        bilersattning = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"Traktamente" => {
                        traktamente = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"PersonaloptionForvarvAndel" => {
                        personaloption_forvarv_andel = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"Arbetsstallenummer" => {
                        arbetsstallenummer = Some(reader.read_text(element.name()).unwrap());
                    }

                    b"Delagare" => {
                        delagare = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"SocialAvgiftsAvtal" => {
                        social_avgifts_avtal = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }
                    b"Inkomstar" => {
                        inkomstar = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Borttag" => {
                        borttag = Some(to_bool(reader.read_text(element.name()).unwrap()).unwrap());
                    }

                    b"Specifikationsnummer" => {
                        specificationsnummer = Some(reader.read_text(element.name()).unwrap().parse().unwrap());
                    }

                    b"InkomsttagareKU10" => {
                        inkomsttagare = Some(InkomsttagareKU10::read(reader, &element)?)
                    }
                    b"UppgiftslamnareKU10" => {
                        uppgiftslamnare = Some(UppgiftslamnareKU10::read(reader, &element)?)
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            kontant_bruttolon_mm,
                            forman_utom_bil_drivmedel,
                            bilforman_utom_drivmedel,
                            drivmedel_vid_bilforman,
                            andra_kostnadsers,
                            underlag_rutarbete,
                            underlag_rotarbete,
                            ers_m_egenavgifter,
                            tjanstepension,
                            ers_ej_soc_avg,
                            ers_ej_soc_avg_ej_jobbavd,
                            forsarskattenamnden,
                            vissa_avdrag,
                            hyresersattning,
                            bostad_smahus,
                            bostad_ej_smahus,
                            forman_har_justerats,
                            forman_som_pension,
                            bilersattning,
                            traktamente,
                            personaloption_forvarv_andel,
                            arbetsstallenummer,
                            delagare,
                            social_avgifts_avtal,
                            inkomstar: inkomstar.ok_or_else(|| MissingField("Inkomstar".to_string()))?,
                            borttag,
                            specifikationsnummer: specificationsnummer.ok_or_else(|| MissingField("Specifikationsnummer".to_string()))?,
                            inkomsttagare: inkomsttagare.ok_or_else(|| MissingField("InkomsttagareKU28".to_string()))?,
                            uppgiftslamnare: uppgiftslamnare.ok_or_else(|| MissingField("UppgiftslamnareKU28".to_string()))?,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UppgiftslamnareKU10<'a> {
    pub uppgiftslamnar_id: Cow<'a, str>,
    pub namn_uppgiftslamnare: Option<Cow<'a, str>>,
}


impl<'a> UppgiftslamnareKU10<'a> {
    fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("UppgiftslamnareKU10").write_inner_content(|w| {
            w.write_node_with_code("UppgiftslamnarId", "201", &self.uppgiftslamnar_id)?;
            w.write_node_with_code("NamnUppgiftslamnare", "202", &self.namn_uppgiftslamnare)?;

            Ok(())
        })?;

        Ok(())
    }
}

impl<'a> UppgiftslamnareKU10<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {
        let mut uppgiftslamnar_id = None;
        let mut namn_uppgiftslamnare = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"UppgiftslamnarId" => {
                        uppgiftslamnar_id = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"NamnUppgiftslamnare" => {
                        namn_uppgiftslamnare = Some(reader.read_text(element.name()).unwrap());
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            uppgiftslamnar_id: uppgiftslamnar_id.ok_or_else(|| MissingField("UppgiftslamnarId".to_string()))?,
                            namn_uppgiftslamnare,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InkomsttagareKU10<'a> {
    pub landskod_tin: Option<Cow<'a, str>>,
    pub inkomsttagare: Option<Cow<'a, str>>,
    pub fornamn: Option<Cow<'a, str>>,
    pub efternamn: Option<Cow<'a, str>>,
    pub gatuadress: Option<Cow<'a, str>>,
    pub postnummer: Option<Cow<'a, str>>,
    pub postort: Option<Cow<'a, str>>,
    pub landskod_postort: Option<Cow<'a, str>>,
    pub fodelsetid: Option<Cow<'a, str>>,
    pub annat_id_nr: Option<Cow<'a, str>>,
    pub org_namn: Option<Cow<'a, str>>,
    pub gatuadress2: Option<Cow<'a, str>>,
    pub fri_adress: Option<Cow<'a, str>>,
    pub tin: Option<Cow<'a, str>>,
}

impl<'a> InkomsttagareKU10<'a> {
    fn write<W>(&self, w: &mut Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {
        w.create_element("InkomsttagareKU10").write_inner_content(|w| {
            w.write_node_with_code("LandskodTIN", "076", &self.landskod_tin)?;
            w.write_node_with_code("Inkomsttagare", "215", &self.inkomsttagare)?;
            w.write_node_with_code("Fornamn", "216", &self.fornamn)?;
            w.write_node_with_code("Efternamn", "217", &self.efternamn)?;
            w.write_node_with_code("Gatuadress", "218", &self.gatuadress)?;
            w.write_node_with_code("Postnummer", "219", &self.postnummer)?;
            w.write_node_with_code("Postort", "220", &self.postort)?;
            w.write_node_with_code("LandskodPostort", "221", &self.landskod_postort)?;
            w.write_node_with_code("Fodelsetid", "222", &self.fodelsetid)?;
            w.write_node_with_code("AnnatIDNr", "224", &self.annat_id_nr)?;
            w.write_node_with_code("OrgNamn", "226", &self.org_namn)?;
            w.write_node_with_code("Gatuadress2", "228", &self.gatuadress2)?;
            w.write_node_with_code("FriAdress", "230", &self.fri_adress)?;
            w.write_node_with_code("TIN", "252", &self.tin)?;

            Ok(())
        })?;

        Ok(())
    }
}

impl<'a> InkomsttagareKU10<'a> {
    fn read(reader: &mut NsReader<&'a [u8]>, tag: &BytesStart) -> Result<Self, DeError> {
        let mut landskod_tin = None;
        let mut inkomsttagare = None;
        let mut fornamn = None;
        let mut efternamn = None;
        let mut gatuadress = None;
        let mut postnummer = None;
        let mut postort = None;
        let mut landskod_postort = None;
        let mut fodelsetid = None;
        let mut annat_id_nr = None;
        let mut org_namn = None;
        let mut gatuadress2 = None;
        let mut fri_adress = None;
        let mut tin = None;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) => match element.local_name().as_ref() {
                    b"LandskodTIN" => {
                        landskod_tin = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Inkomsttagare" => {
                        inkomsttagare = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Fornamn" => {
                        fornamn = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Efternamn" => {
                        efternamn = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Gatuadress" => {
                        gatuadress = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Postnummer" => {
                        postnummer = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Postort" => {
                        postort = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"LandskodPostort" => {
                        landskod_postort = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Fodelsetid" => {
                        fodelsetid = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"AnnatIDNr" => {
                        annat_id_nr = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"OrgNamn" => {
                        org_namn = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"Gatuadress2" => {
                        gatuadress2 = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"FriAdress" => {
                        fri_adress = Some(reader.read_text(element.name()).unwrap());
                    }
                    b"TIN" => {
                        tin = Some(reader.read_text(element.name()).unwrap());
                    }
                    &_ => return Err(UnexpectedElement(std::str::from_utf8(element.name().as_ref()).unwrap().to_string()))
                }
                Event::End(element) => {
                    if element.name() == tag.name() {
                        return Ok(Self {
                            landskod_tin,
                            inkomsttagare,
                            fornamn,
                            efternamn,
                            gatuadress,
                            postnummer,
                            postort,
                            landskod_postort,
                            fodelsetid,
                            annat_id_nr,
                            org_namn,
                            gatuadress2,
                            fri_adress,
                            tin,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{Arendeinformation, Avsandare, Blankett, Blankettgemensamt, from_str, Kontaktperson, Kontrolluppgift, TekniskKontaktperson, to_string, Uppgiftslamnare};
    use crate::KontrolluppgiftType::KU10;
    use crate::ku10::{InkomsttagareKU10, KU10Type, UppgiftslamnareKU10};

    #[test]
    fn ku10_is_read() {
        let xml = fs::read_to_string("EXEMPELFIL KONTROLLUPPGIFT FÖR ARBETSGIVARE MED SOCIALAVGIFTSAVTAL (KU10)_2022.xml").unwrap();

        let parsed = from_str(&*xml).unwrap();
        let unparsed = to_string(&parsed).unwrap();
        let parsed2 = from_str(&*unparsed).unwrap();
        assert_eq!(parsed, parsed2);
    }

    #[test]
    fn ku10_is_parsed_to_and_back() {
        let ku28 = Kontrolluppgift {
            avsandare: Avsandare {
                teknisk_kontaktperson: TekniskKontaktperson {
                    ..Default::default()
                },
                ..Default::default()
            },
            blankettgemensamt: Blankettgemensamt {
                uppgiftslamnare: Uppgiftslamnare {
                    kontaktperson: Kontaktperson {
                        ..Default::default()
                    },
                    ..Default::default()
                }
            },
            blanketter: vec![
                Blankett {
                    nummer: 0,
                    arendeinformation: Arendeinformation {
                        ..Default::default()
                    },
                    blankettinnehall: KU10(KU10Type {
                        kontant_bruttolon_mm: Some(1),
                        forman_utom_bil_drivmedel: Some(2),
                        bilforman_utom_drivmedel: Some(3),
                        drivmedel_vid_bilforman: Some(4),
                        andra_kostnadsers: Some(5),
                        underlag_rutarbete: Some(6),
                        underlag_rotarbete: Some(7),
                        ers_m_egenavgifter: Some(8),
                        tjanstepension: Some(9),
                        ers_ej_soc_avg: Some(10),
                        ers_ej_soc_avg_ej_jobbavd: Some(11),
                        forsarskattenamnden: Some(12),
                        vissa_avdrag: Some(13),
                        hyresersattning: Some(14),
                        bostad_smahus: Some(true),
                        bostad_ej_smahus: Some(false),
                        forman_har_justerats: Some(true),
                        forman_som_pension: Some(false),
                        bilersattning: Some(true),
                        traktamente: Some(false),
                        personaloption_forvarv_andel: Some(true),
                        arbetsstallenummer: Some("12".into()),
                        delagare: Some(false),
                        social_avgifts_avtal: Some(true),
                        inkomstar: "2022".into(),
                        borttag: Some(false),

                        specifikationsnummer: 5,
                        inkomsttagare: InkomsttagareKU10 {
                            landskod_tin: Some("landskod tin".into()),
                            inkomsttagare: Some("202301062382".into()),
                            fornamn: Some("Test".into()),
                            efternamn: Some("Testsson".into()),
                            gatuadress: Some("Gata".into()),
                            postnummer: Some("7456".into()),
                            postort: Some("Postort".into()),
                            landskod_postort: Some("FI".into()),
                            fodelsetid: Some("20230106".into()),
                            annat_id_nr: Some("202".into()),
                            org_namn: Some("Organization".into()),
                            gatuadress2: Some("Gata2".into()),
                            fri_adress: Some("Storgatan 3".into()),
                            tin: Some("Tin".into()),
                        },
                        uppgiftslamnare: UppgiftslamnareKU10 {
                            uppgiftslamnar_id: "165599990602".into(),
                            namn_uppgiftslamnare: Some("Foretag 1".into()),
                        },
                    }),
                }
            ],
        };
        let unparsed = to_string(&ku28).unwrap();
        let re_parsed = from_str(&*unparsed).unwrap();
        assert_eq!(ku28, re_parsed);
    }
}
