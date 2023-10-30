# Check if current folder is up-to-date with github, and if so, publish it to crates.io.
PREV_PWD=$(pwd)

git fetch

if [ $(git rev-parse HEAD) != $(git rev-parse @{u}) ]; then
	echo "Local repository is not up-to-date with github. Please push your changes first."
	exit 1
fi

TMPDIR=$(mktemp -d)
git clone https://github.com/Brian3647/qli $TMPDIR
cd $TMPDIR
cargo publish

# Clean-up

cd $PREV_PWD
rm -rf $TMPDIR
