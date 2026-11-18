## to share files from iphone to my laptop using ish

# On my iphone

1. to create a mount folder(to create a label to plug the actual folder from files app)
'''
mkdir /mnt/xyz 
cd /mnt/xyz
'''

2. then to mount the folder
'''
mount -t ios . /mnt/xyz
'''
3.after choosing the folder start a python server

'''
python3 -m http.server 8000
'''
