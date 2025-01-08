set ip=

docker save lol-draft:latest > ./lol-draft.tar

scp lol-draft.tar matthewweisfeld@%ip%:~/Documents/G3Tech/Server/LolDraft

scp compose.yml matthewweisfeld@%ip%:~/Documents/G3Tech/Server/LolDraft
scp deploy/remoteDeploy.sh matthewweisfeld@%ip%:~/Documents/G3Tech/Server/LolDraft

ssh matthewweisfeld@%ip% chmod +x /home/matthewweisfeld/Documents/G3Tech/Server/LolDraft/remoteDeploy.sh
ssh matthewweisfeld@%ip% /home/matthewweisfeld/Documents/G3Tech/Server/LolDraft/remoteDeploy.sh