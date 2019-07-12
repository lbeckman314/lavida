targets=($(rustup target list --installed))
echo "Build targets:"
i=1
for target in "${targets[@]}"
do
    echo -e "\t$i: $target"
    ((i+=1))
done

i=1
for target in "${targets[@]}"
do
    echo "Building $i/${#targets[@]}: $target"
    cargo build --target="$target" --release
    ((i+=1))
done
