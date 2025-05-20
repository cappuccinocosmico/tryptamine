For this entire repository could you make a kubernetes helm chart that serves the blog. There should be code for building and pushing the docker image to a remote repository in `build_docker_images.sh`. 

This kubernetes cluster should 

- include 2 copies of the main docker image 
- host it on a url an.mycor.io
- deal with automatically getting and refreshing https certificates from letsencrypt.

I have had a bunch of issues with kubernetes in the past and trying to get it to work. Could you also make it possible to use the helm chart locally, and try to test that everything works (aside from the fact that fetching the certificates might fail). Could you throw up everything into an architecture document under prompts/helm-ideas.md
