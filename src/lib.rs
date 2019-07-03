//! Based on Jon Gjengset's live-coding series: https://youtu.be/Zdudg5TV9i4
use std::collections::HashMap;
use std::io;

pub struct MachineSetup<F> {
    instance_type: String,
    ami: String,
    setup: F,
}
struct SshConnection;
struct Machine {
    ssh: SshConnection,
    instance_type: String,
    ip: String,
    dns: String,
}

impl MachineSetup {
    pub fn new<F>(instance_type: String, ami: String, setup: F) -> Self
    where
        F: Fn(&mut SshConnection) -> io::Result<()>,
    {
        MachineSetup {
            instance_type,
            ami,
            setup,
        }
    }
}

struct BurstBuilder {
    descriptors: HashMap<String, (MachineSetup, u32)>,
}

impl Default for BurstBuilder {
    fn default() -> Self {
        BurstBuilder {
            descriptors: Default::default(),
        }
    }
}

impl BurstBuilder {
    pub fn add_set(&mut self, name: String, number: u32, setup: MachineSetup) {
        // TODO: what if name is already in use?
        self.descriptors.insert(name, (setup, number));
    }

    pub fn run<F>()
    where
        F: FnOnce(HashMap<String, &mut [Machine]>) -> io::Result<()>,
    {
        // 1. issue spot requests
        // 2. wait for instances to come up
        // - once an instance is ready, run setup closure
        // 3. wait until all instances are up & setups have been run
        // 4. stop spot requests
        // 5. invoke F with Machine descriptors
        // 6. terminate all instances
    }
}

fn main() {
    let mut b = BurstBuilder::default();
    b.add_set(
        "server",
        1,
        MachineSetup::new("t2.micro", "ami-e18aa89b", |ssh| {
            ssh.exec("sudo yum install htop");
            // yum install apache
        }),
    );

    b.add_set(
        "client",
        2,
        MachineSetup::new("t2.micro", "ami-e18aa89b", |ssh| {
            ssh.exec("sudo yum install htop");
            // git clone...
        }),
    );

    b.run(|vms: HashMap<String, MachineSet>| {
        let server_ip = vms["server"][0].ip;
        let cmd = format!("ping {}", server_ip);
        vms["client"].for_each_parallel(|client| {
            client.exec(cmd);
        })
    });
}
