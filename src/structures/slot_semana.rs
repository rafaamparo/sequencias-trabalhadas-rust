pub struct SlotSemanal {
    pub dia_semana: u8,
    pub hora_inicio: u8,
    pub hora_fim: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criar_slot_semanal() {
        let slot = SlotSemanal {
            dia_semana: 3,
            hora_inicio: 10,
            hora_fim: 14,
        };

        assert_eq!(slot.dia_semana, 3);
        assert_eq!(slot.hora_inicio, 10);
        assert_eq!(slot.hora_fim, 14);
    }

    #[test]
    fn test_duracao_slot() {
        let slot = SlotSemanal {
            dia_semana: 2,
            hora_inicio: 9,
            hora_fim: 13,
        };

        let duracao = slot.hora_fim - slot.hora_inicio;
        assert_eq!(duracao, 4); // O slot tem duração de 4 horas
    }
}