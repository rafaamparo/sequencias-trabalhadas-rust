use developers::structures::desenvolvedor::Desenvolvedor;
use developers::structures::slot_semanal::SlotSemanal;
use std::collections::HashMap;

#[test]
fn test_integracao_desenvolvedores_slots() {
    let mut desenvolvedores: HashMap<String, Desenvolvedor> = HashMap::new();

    // Criar desenvolvedores com diferentes padrões de slots
    let mut dev1 = Desenvolvedor::new("Dev1".to_string());
    let mut dev2 = Desenvolvedor::new("Dev2".to_string());
    let mut dev3 = Desenvolvedor::new("Dev3".to_string());

    // Dev1: Muitas horas interruptas
    dev1.adicionar_slot(SlotSemanal {
        dia_semana: 1,
        hora_inicio: 9,
        hora_fim: 13,
    });
    dev1.adicionar_slot(SlotSemanal {
        dia_semana: 1,
        hora_inicio: 13,
        hora_fim: 17,
    });

    // Dev2: Muitos slots interruptos
    dev2.adicionar_slot(SlotSemanal {
        dia_semana: 2,
        hora_inicio: 9,
        hora_fim: 10,
    });
    dev2.adicionar_slot(SlotSemanal {
        dia_semana: 2,
        hora_inicio: 10,
        hora_fim: 11,
    });
    dev2.adicionar_slot(SlotSemanal {
        dia_semana: 2,
        hora_inicio: 11,
        hora_fim: 12,
    });
    dev2.adicionar_slot(SlotSemanal {
        dia_semana: 2,
        hora_inicio: 12,
        hora_fim: 13,
    });
    dev2.adicionar_slot(SlotSemanal {
        dia_semana: 2,
        hora_inicio: 13,
        hora_fim: 14,
    });

    // Dev3: Slots não consecutivos
    dev3.adicionar_slot(SlotSemanal {
        dia_semana: 3,
        hora_inicio: 9,
        hora_fim: 11,
    });
    dev3.adicionar_slot(SlotSemanal {
        dia_semana: 3,
        hora_inicio: 13,
        hora_fim: 15,
    });
    dev3.adicionar_slot(SlotSemanal {
        dia_semana: 4,
        hora_inicio: 10,
        hora_fim: 12,
    });

    desenvolvedores.insert(dev1.nome.clone(), dev1);
    desenvolvedores.insert(dev2.nome.clone(), dev2);
    desenvolvedores.insert(dev3.nome.clone(), dev3);

    // Testes de integração

    // Teste para encontrar desenvolvedor com mais horas interruptas
    let mut max_horas = 0;
    let mut dev_max_horas = String::new();

    for (nome, dev) in &desenvolvedores {
        let (horas, _) = dev.horas_interruptas_trabalhadas();
        if horas > max_horas {
            max_horas = horas;
            dev_max_horas = nome.clone();
        }
    }

    assert_eq!(dev_max_horas, "Dev1");
    assert_eq!(max_horas, 8); // 8 horas interruptas (9-17)

    // Teste para encontrar desenvolvedor com mais slots interruptos
    let mut max_slots = 0;
    let mut dev_max_slots = String::new();

    for (nome, dev) in &desenvolvedores {
        let (_, slots) = dev.horas_interruptas_trabalhadas();
        if slots > max_slots {
            max_slots = slots;
            dev_max_slots = nome.clone();
        }
    }

    assert_eq!(dev_max_slots, "Dev2");
    assert_eq!(max_slots, 5); // 5 slots consecutivos
}