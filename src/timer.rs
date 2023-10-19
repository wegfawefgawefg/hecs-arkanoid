pub struct Timer {
    pub interval: u32, // frames
    pub countdown: u32,
}

impl Timer {
    pub fn new(interval: u32, initial_countdown: u32) -> Self {
        Self {
            interval,
            countdown: initial_countdown,
        }
    }
}

pub struct TypedTimer<T> {
    timer: Timer,
    _marker: std::marker::PhantomData<T>,
}
impl<T> TypedTimer<T> {
    pub fn new(spawn_interval: u32, initial_countdown: u32) -> Self {
        Self {
            timer: Timer::new(spawn_interval, initial_countdown),
            _marker: std::marker::PhantomData,
        }
    }
    pub fn get_countdown(&self) -> u32 {
        self.timer.countdown
    }
    pub fn reset(&mut self) {
        self.timer.countdown = self.timer.interval;
    }

    pub fn step(&mut self) {
        if self.timer.countdown > 0 {
            self.timer.countdown -= 1;
        }
    }
}
pub struct ForAsteroidSpawning;
pub struct ForGunSpawning;
pub struct ForEnemySpawning;
pub type AsteroidSpawnTimer = TypedTimer<ForAsteroidSpawning>;
pub type GunSpawnTimer = TypedTimer<ForGunSpawning>;
pub type EnemySpawnTimer = TypedTimer<ForEnemySpawning>;
