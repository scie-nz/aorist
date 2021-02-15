kubectl create secret generic aorist-deploy-key --from-file=ssh-privatekey=/home/bogdan/.ssh/aorist_devserver --from-file=ssh-publickey=/home/bogdan/.ssh/aorist_devserver.pub
