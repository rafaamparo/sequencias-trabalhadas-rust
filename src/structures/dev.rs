use crate::structures::slot_semanal::SlotSemanal;

pub struct Desenvolvedor {
    pub nome: String,
    pub slots_semana: Vec<SlotSemanal>,
}

impl Desenvolvedor {
    pub fn new(nome: String) -> Self {
        Desenvolvedor {
            nome,
            slots_semana: Vec::new(),
        }
    }

    pub fn adicionar_slot(&mut self, slot: SlotSemanal) {
        self.slots_semana.push(slot);
    }

    pub fn horas_interruptas_trabalhadas(&self) -> (u8, usize) {
        let mut maior_num_slots = 0;
        let mut maior_total_horas = 0;

        let mut total_horas = 0;
        let mut num_slots = 0;

        let mut slot_anterior: Option<&SlotSemanal> = None;
        for slot in &self.slots_semana {
            let horas_slot = if slot.hora_fim >= slot.hora_inicio {
                slot.hora_fim - slot.hora_inicio
            } else {
                24 + slot.hora_fim - slot.hora_inicio
            };

            if let Some(prev) = slot_anterior {
                let is_consecutive = (prev.dia_semana == slot.dia_semana
                    && prev.hora_fim == slot.hora_inicio)
                    || (prev.hora_fim == 0 && slot.hora_inicio == 0);

                if is_consecutive {
                    num_slots += 1;
                    total_horas += horas_slot as usize;
                } else {
                    num_slots = 1;
                    total_horas = horas_slot as usize;
                }
            } else {
                num_slots = 1;
                total_horas = horas_slot as usize;
            }

            if num_slots > maior_num_slots {
                maior_num_slots = num_slots;
            }
            if total_horas > maior_total_horas {
                maior_total_horas = total_horas;
            }

            slot_anterior = Some(slot);
        }

        (maior_total_horas as u8, maior_num_slots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_desenvolvedor() {
        let dev = Desenvolvedor::new("Carlos".to_string());
        assert_eq!(dev.nome, "Carlos");
        assert!(dev.slots_semana.is_empty());
    }

    #[test]
    fn test_adicionar_slot() {
        let mut dev = Desenvolvedor::new("Carlos".to_string());
        let slot = SlotSemanal {
            dia_semana: 1,
            hora_inicio: 9,
            hora_fim: 12,
        };
        dev.adicionar_slot(slot);
        assert_eq!(dev.slots_semana.len(), 1);
        assert_eq!(dev.slots_semana[0].dia_semana, 1);
        assert_eq!(dev.slots_semana[0].hora_inicio, 9);
        assert_eq!(dev.slots_semana[0].hora_fim, 12);
    }

    #[test]
    fn test_horas_interruptas_sem_slots() {
        let dev = Desenvolvedor::new("Carlos".to_string());
        let (horas, num_slots) = dev.horas_interruptas_trabalhadas();
        assert_eq!(horas, 0);
        assert_eq!(num_slots, 0);
    }

    #[test]
    fn test_horas_interruptas_com_slots_consecutivos() {
        let mut dev = Desenvolvedor::new("Carlos".to_string());

        // Adicionar 3 slots consecutivos no mesmo dia
        dev.adicionar_slot(SlotSemanal {
            dia_semana: 1,
            hora_inicio: 9,
            hora_fim: 11,
        });
        dev.adicionar_slot(SlotSemanal {
            dia_semana: 1,
            hora_inicio: 11,
            hora_fim: 13,
        });
        dev.adicionar_slot(SlotSemanal {
            dia_semana: 1,
            hora_inicio: 13,
            hora_fim: 15,
        });

        // Adicionar um slot não consecutivo
        dev.adicionar_slot(SlotSemanal {
            dia_semana: 2,
            hora_inicio: 10,
            hora_fim: 12,
        });

        let (horas, num_slots) = dev.horas_interruptas_trabalhadas();
        assert_eq!(horas, 6); // 3 slots consecutivos totalizando 6 horas
        assert_eq!(num_slots, 3); // 3 slots consecutivos
    }

    #[test]
    fn test_horas_interruptas_com_slots_nao_consecutivos() {
        let mut dev = Desenvolvedor::new("Carlos".to_string());

        // Adicionar slots não consecutivos
        dev.adicionar_slot(SlotSemanal {
            dia_semana: 1,
            hora_inicio: 9,
            hora_fim: 11,
        });
        dev.adicionar_slot(SlotSemanal {
            dia_semana: 2,
            hora_inicio: 11,
            hora_fim: 14,
        });
        dev.adicionar_slot(SlotSemanal {
            dia_semana: 3,
            hora_inicio: 10,
            hora_fim: 15,
        });

        let (horas, num_slots) = dev.horas_interruptas_trabalhadas();
        assert_eq!(horas, 5); // O slot mais longo tem 5 horas (10 às 15)
        assert_eq!(num_slots, 1); // Apenas 1 slot consecutivo (ele mesmo)
    }
}