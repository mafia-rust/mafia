use mafia_server::game::components::transporting::TransportPriority;

#[test]
fn test_transport_ord() {
    assert!(TransportPriority::Transporting <= TransportPriority::Transporting);
    assert!(TransportPriority::Transporting <= TransportPriority::Warping);
    assert!(TransportPriority::Transporting <= TransportPriority::Bodyguard);
    assert!(TransportPriority::Transporting <= TransportPriority::None);
    assert!(TransportPriority::Warping <= TransportPriority::Warping);
    assert!(TransportPriority::Warping <= TransportPriority::Bodyguard);
    assert!(TransportPriority::Warping <= TransportPriority::None);
    assert!(TransportPriority::Bodyguard <= TransportPriority::Bodyguard);
    assert!(TransportPriority::Bodyguard <= TransportPriority::None);
    assert!(TransportPriority::None <= TransportPriority::None);
}