use specs::VecStorage;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Health(u32);


#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Damage(u32);
