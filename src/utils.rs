pub trait State {
    type Input;
    fn take_input(&mut self, input: Self::Input);
}

pub mod cycle {
    #[derive(Debug, Clone)]
    pub struct Cycle {
        phase: Phase,
        active: bool,
        windup_time: Option<f32>,
        cooldown_time: Option<f32>,
        time: f32,
    }

    impl Cycle {
        pub fn new(windup_time: Option<f32>, cooldown_time: Option<f32>) -> Self {
            Cycle {
                phase: Phase::Inactive,
                active: false,
                windup_time,
                cooldown_time,
                time: 0.0,
            }
        }
        pub fn get_phase_change(&mut self, dt: f32) -> Option<PhaseChange> {
            let phase_change = match self.phase {
                Phase::Inactive => {
                    if self.active {
                        if self.windup_time != None {
                            self.begin_phase(Phase::Windup);
                            Some(PhaseChange::BeginWindup)
                        } else {
                            self.begin_phase(Phase::Cooldown);
                            Some(PhaseChange::Trigger)
                        }
                    } else {
                        None
                    }
                }
                Phase::Windup => {
                    // if this statement is here, you can cancel out of a wind up by being inactive
                    /*
                    if !self.active {
                        self.begin_phase(Phase::Inactive);
                        Some(PhaseChange::CancelWindup)
                    } else 
                    */
                    if self.time > self.windup_time.unwrap_or_default() {
                        if self.cooldown_time != None {
                            self.begin_phase(Phase::Cooldown);
                        } else {
                            self.begin_phase(Phase::Inactive);
                        }
                        Some(PhaseChange::Trigger)
                    } else {
                        None
                    }
                }
                Phase::Cooldown => {
                    if self.time > self.cooldown_time.unwrap_or_default() {
                        self.begin_phase(Phase::Inactive);
                        Some(PhaseChange::EndCooldown)
                    } else {
                        None
                    }
                }
            };
            if self.phase != Phase::Inactive {
                self.time += dt;
            }
            phase_change
        }
        fn begin_phase(&mut self, phase: Phase) {
            self.phase = phase;
            self.time = 0.0;
        }
        pub fn set_active(&mut self, active: bool) {
            self.active = active;
        }
        pub fn get_phase(&self) -> Phase {
            self.phase
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Phase {
        Inactive,
        Windup,
        Cooldown,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PhaseChange {
        BeginWindup,
        CancelWindup,
        Trigger,
        EndCooldown,
    }
}
