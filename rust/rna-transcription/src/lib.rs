#[derive(Debug, PartialEq)]
pub struct DNA {
    pub bases: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    pub bases: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, c) in dna.chars().enumerate() {
            let v = match c {
                'A' | 'C' | 'G' | 'T' => Ok(()),
                _ => Err(()),
            };

            if !v.is_ok() {
                return Err(i);
            }
        }

        Ok(DNA {
            bases: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> RNA {
        let mut rna = String::new();
        for c in self.bases.chars() {
            rna.push(match c {
                'A' => 'U',
                'G' => 'C',
                'T' => 'A',
                'C' => 'G',
                _ => panic!("At the disco"),
            });
        }

        RNA::new(&rna).unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, c) in rna.chars().enumerate() {
            let v = match c {
                'A' | 'C' | 'G' | 'U' => Ok(()),
                _ => Err(()),
            };

            if !v.is_ok() {
                return Err(i);
            }
        }

        Ok(RNA {
            bases: rna.to_string(),
        })
    }
}
