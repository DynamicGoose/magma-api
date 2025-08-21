pub trait AppSchedule {}

pub struct Startup;

impl AppSchedule for Startup {}

pub struct PreUpdate;

impl AppSchedule for PreUpdate {}

pub struct Update;

impl AppSchedule for Update {}

pub struct PostUpdate;

impl AppSchedule for PostUpdate {}
