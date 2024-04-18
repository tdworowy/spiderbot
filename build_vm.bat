mkdir output
vagrant ssh -c "mkdir spiderbot"
vagrant scp build.sh default:/home/vagrant/

vagrant scp .cargo default:/home/vagrant/spiderbot
vagrant scp src default:/home/vagrant/spiderbot
vagrant scp Cargo.lock default:/home/vagrant/spiderbot
vagrant scp Cargo.toml default:/home/vagrant/spiderbot

vagrant ssh -c "/home/vagrant/build.sh"
vagrant scp default:/home/vagrant/spiderbot/target/armv7-unknown-linux-gnueabihf/debug/spiderbot output/spiderbot