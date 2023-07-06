use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use itybity::{IntoBitIterator, ToBits};
use mpz_core::Block;
use mpz_ot_core::{chou_orlandi, kos};
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;

fn chou_orlandi(c: &mut Criterion) {
    let mut group = c.benchmark_group("chou_orlandi");
    for n in [128, 256, 1024] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let msgs = vec![[Block::ONES; 2]; n];
            let mut rng = ChaCha12Rng::from_entropy();
            let mut choices = vec![0u8; n / 8];
            rng.fill_bytes(&mut choices);
            b.iter(|| {
                let sender = chou_orlandi::Sender::default();
                let receiver = chou_orlandi::Receiver::default();

                let (sender_setup, sender) = sender.setup();
                let (receiver_setup, mut receiver) = receiver.setup(sender_setup);
                let mut sender = sender.receive_setup(receiver_setup).unwrap();

                let receiver_payload = receiver.receive_random(choices.as_slice());
                let sender_payload = sender.send(&msgs, receiver_payload).unwrap();
                black_box(receiver.receive(sender_payload).unwrap())
            })
        });
    }
}

fn kos(c: &mut Criterion) {
    let mut group = c.benchmark_group("kos");
    for n in [1024, 262144] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let msgs = vec![[Block::ONES; 2]; n];
            let mut rng = ChaCha12Rng::from_entropy();
            let mut choices = vec![0u8; n / 8];
            rng.fill_bytes(&mut choices);
            let choices = choices.into_lsb0_vec();
            let delta = Block::random(&mut rng);
            let chi_seed = Block::random(&mut rng);

            let receiver_seeds: [[Block; 2]; 128] = std::array::from_fn(|_| [rng.gen(), rng.gen()]);
            let sender_seeds: [Block; 128] = delta
                .iter_lsb0()
                .zip(receiver_seeds)
                .map(|(b, seeds)| if b { seeds[1] } else { seeds[0] })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            b.iter(|| {
                let sender = kos::Sender::new(kos::SenderConfig::default());
                let receiver = kos::Receiver::new(kos::ReceiverConfig::default());

                let mut sender = sender.base_setup(delta, sender_seeds);
                let mut receiver = receiver.base_setup(receiver_seeds);

                let receiver_setup = receiver.extend(choices.len() + 256);
                sender.extend(msgs.len() + 256, receiver_setup).unwrap();

                let receiver_check = receiver.check(chi_seed);
                sender.check(chi_seed, receiver_check).unwrap();

                let derandomize = receiver.derandomize(&choices);
                let payload = sender.send(&msgs, derandomize).unwrap();
                let received = receiver.receive(payload).unwrap();

                black_box(received)
            })
        });
    }
}

criterion_group! {
    name = chou_orlandi_benches;
    config = Criterion::default().sample_size(50);
    targets = chou_orlandi
}

criterion_group! {
    name = kos_benches;
    config = Criterion::default().sample_size(50);
    targets = kos
}

criterion_main!(chou_orlandi_benches, kos_benches);
