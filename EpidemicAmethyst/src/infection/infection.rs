
pub enum VirusType {
    Stardand
}

pub enum BacteriaType {
    Standard
}
pub enum Disease {
    Virus(VirusType),
    Bacteria(BacteriaType)
}