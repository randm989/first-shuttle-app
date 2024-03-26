Successfully was able to get docker to run which is necessary for 
cargo shuttle run
to execute locally

But you can't run as the plain user, you must open a shell via the command
`newgrp docker`
and then run shuttle locally in order for docker to be satisfied


I was able to solve this recurring annoyance by instead granting my user readwrite permissions to the docker directory with the following command:
`sudo setfacl -m user:$USER:rw /var/run/docker.sock`

