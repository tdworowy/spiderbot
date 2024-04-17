mkdir output
vagrant scp build.sh default:/home/vagrant/
vagrant ssh -c "/home/vagrant/build.sh"
vagrant scp default:/home/vagrant/spiderbot/target/armv7-unknown-linux-gnueabihf/debug/spiderbot output/spiderbot