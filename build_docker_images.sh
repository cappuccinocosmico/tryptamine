#!/bin/bash
set -e

# Global variables
remote_target=""
prod_commit=""
nightly_commit=""

function process_branch() {
    local branch=$1
    local commit_hash=$2  # Optional commit hash

    echo "Processing branch: $branch"
    sudo mkdir -p /mycorrhiza
    sudo chmod 777 -R /mycorrhiza
    cd /mycorrhiza
    
    if [ ! -d "/mycorrhiza/tryptamine" ]; then
        git clone https://github.com/cappuccinocosmico/tryptamine
        git config --global --add safe.directory /mycorrhiza/tryptamine
    fi
    
    cd tryptamine

    git clean -fd
    git fetch
    git reset --hard HEAD
    git clean -fd
    # Checkout specific commit or update branch
    if [ -n "$commit_hash" ]; then
        git checkout "$commit_hash"
        echo "Checked out specific commit: $commit_hash"
    else
        git switch "$branch"
        git reset --hard origin/"$branch"
        echo "Updated branch $branch to latest"
    fi

    local current_hash=$(git rev-parse HEAD)
    echo "Current commit hash: $current_hash"
    
    echo "Rebuilding and deploying images..."
    
    # Build and push Docker images
    sudo docker build -t "fractalhuman1/tryptamine-blog:${current_hash}" --platform linux/amd64 --file ./blog/Dockerfile ./blog/

    sudo docker push "fractalhuman1/tryptamine-blog:${current_hash}"

    # Update docker-compose.yml on the server
    # Set deployment variables based on environment
    local deploy_flag="prod"
    local deploy_host="an.mycor.io"


    # ssh "root@${deploy_host}" "run the helm upgrade command"
}

# Parse command line arguments
is_prod=false
while [[ $# -gt 0 ]]; do
    case "$1" in
        --commit)
            commit="$2"
            shift 2
            ;;
        --commit=*)
            commit="${1#*=}"
            shift
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

if [[  -n "$commit" ]]; then
    process_branch "main" "$commit"
else
    process_branch "main"
    # process_branch "release"
fi
#!/bin/bash
set -e


