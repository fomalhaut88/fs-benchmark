use rand::prelude::*;


const COUNT: usize = 100000;
const BLOCK_SIZE: usize = 64;


fn test_std() -> Result<(), std::io::Error> {
    use std::io::{Seek, Read, Write};

    let file_path = "./tmp/file-std";

    // Ensure removing the file
    if std::fs::exists(file_path)? {
        std::fs::remove_file(file_path)?;
    }

    // Create a file
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    // Randomness
    let mut rng = rand::rng();

    println!("Test for std:");

    // Case 1. Push bytes
    let tm = std::time::Instant::now();
    for _ in 0..COUNT {
        let block = rng.random::<[u8; BLOCK_SIZE]>();
        file.seek(std::io::SeekFrom::End(0))?;
        file.write_all(&block)?;
        file.flush()?;
    }
    println!("Case 1. Push bytes:\t{:?}", tm.elapsed());

    // Case 2. Iterate blocks
    let tm = std::time::Instant::now();
    file.seek(std::io::SeekFrom::Start(0))?;
    let mut block = [0u8; BLOCK_SIZE];
    for _ in 0..COUNT {
        file.read_exact(&mut block)?;
    }
    println!("Case 2. Iterate blocks:\t{:?}", tm.elapsed());

    // Case 3. Random read
    let tm = std::time::Instant::now();
    let mut block = [0u8; BLOCK_SIZE];
    for _ in 0..COUNT {
        let pos = rng.random_range(0..COUNT) as u64;
        file.seek(std::io::SeekFrom::Start(pos * BLOCK_SIZE as u64))?;
        file.read_exact(&mut block)?;
    }
    println!("Case 3. Random read:\t{:?}", tm.elapsed());

    // Case 4. Random update
    let tm = std::time::Instant::now();
    for _ in 0..COUNT {
        let block = rng.random::<[u8; BLOCK_SIZE]>();
        let pos = rng.random_range(0..COUNT) as u64;
        file.seek(std::io::SeekFrom::Start(pos * BLOCK_SIZE as u64))?;
        file.write_all(&block)?;
        file.flush()?;
    }
    println!("Case 4. Random update:\t{:?}", tm.elapsed());

    println!("");

    // Return
    Ok(())
}


fn test_std_ext() -> Result<(), std::io::Error> {
    use std::os::unix::prelude::FileExt;

    let file_path = "./tmp/file-std-ext";

    // Ensure removing the file
    if std::fs::exists(file_path)? {
        std::fs::remove_file(file_path)?;
    }

    // Create a file
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    // Randomness
    let mut rng = rand::rng();

    println!("Test for std-ext:");

    // Case 1. Push bytes
    let tm = std::time::Instant::now();
    let mut pos = 0;
    for _ in 0..COUNT {
        let block = rng.random::<[u8; BLOCK_SIZE]>();
        file.write_all_at(&block, pos)?;
        pos += BLOCK_SIZE as u64;
    }
    println!("Case 1. Push bytes:\t{:?}", tm.elapsed());

    // Case 2. Iterate blocks
    let tm = std::time::Instant::now();
    let mut block = [0u8; BLOCK_SIZE];
    let mut pos = 0;
    for _ in 0..COUNT {
        file.read_exact_at(&mut block, pos)?;
        pos += BLOCK_SIZE as u64;
    }
    println!("Case 2. Iterate blocks:\t{:?}", tm.elapsed());

    // Case 3. Random read
    let tm = std::time::Instant::now();
    let mut block = [0u8; BLOCK_SIZE];
    for _ in 0..COUNT {
        let pos = rng.random_range(0..COUNT) as u64;
        file.read_exact_at(&mut block, pos)?;
    }
    println!("Case 3. Random read:\t{:?}", tm.elapsed());

    // Case 4. Random update
    let tm = std::time::Instant::now();
    for _ in 0..COUNT {
        let block = rng.random::<[u8; BLOCK_SIZE]>();
        let pos = rng.random_range(0..COUNT) as u64;
        file.write_all_at(&block, pos)?;
    }
    println!("Case 4. Random update:\t{:?}", tm.elapsed());

    println!("");

    // Return
    Ok(())
}


fn test_tokio() -> Result<(), std::io::Error> {
    use tokio::io::{AsyncSeekExt, AsyncReadExt, AsyncWriteExt};

    let file_path = "./tmp/file-tokio";

    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(async {
        // Ensure removing the file
        if tokio::fs::try_exists(file_path).await? {
            tokio::fs::remove_file(file_path).await?;
        }

        // Create a file
        let mut file = tokio::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .await?;

        // Randomness
        let mut rng = rand::rng();

        println!("Test for tokio:");

        // Case 1. Push bytes
        let tm = std::time::Instant::now();
        for _ in 0..COUNT {
            let block = rng.random::<[u8; BLOCK_SIZE]>();
            file.seek(std::io::SeekFrom::End(0)).await?;
            file.write_all(&block).await?;
            file.flush().await?;
        }
        println!("Case 1. Push bytes:\t{:?}", tm.elapsed());

        // Case 2. Iterate blocks
        let tm = std::time::Instant::now();
        file.seek(std::io::SeekFrom::Start(0)).await?;
        let mut block = [0u8; BLOCK_SIZE];
        for _ in 0..COUNT {
            file.read_exact(&mut block).await?;
        }
        println!("Case 2. Iterate blocks:\t{:?}", tm.elapsed());

        // Case 3. Random read
        let tm = std::time::Instant::now();
        let mut block = [0u8; BLOCK_SIZE];
        for _ in 0..COUNT {
            let pos = rng.random_range(0..COUNT) as u64;
            file.seek(std::io::SeekFrom::Start(pos * BLOCK_SIZE as u64)).await?;
            file.read_exact(&mut block).await?;
        }
        println!("Case 3. Random read:\t{:?}", tm.elapsed());

        // Case 4. Random update
        let tm = std::time::Instant::now();
        for _ in 0..COUNT {
            let block = rng.random::<[u8; BLOCK_SIZE]>();
            let pos = rng.random_range(0..COUNT) as u64;
            file.seek(std::io::SeekFrom::Start(pos * BLOCK_SIZE as u64)).await?;
            file.write_all(&block).await?;
            file.flush().await?;
        }
        println!("Case 4. Random update:\t{:?}", tm.elapsed());

        println!("");

        // Return
        Ok(())
    })
}


fn test_tokio_uring() -> Result<(), std::io::Error> {
    let file_path = "./tmp/file-tokio-uring";

    tokio_uring::start(async {
        // Ensure removing the file
        if tokio_uring::fs::statx(file_path).await.is_ok() {
            tokio_uring::fs::remove_file(file_path).await?;
        }

        // Create a file
        let file = tokio_uring::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .await?;

        // Randomness
        let mut rng = rand::rng();

        println!("Test for tokio-uring:");

        // Case 1. Push bytes
        let tm = std::time::Instant::now();
        let mut pos = 0;
        for _ in 0..COUNT {
            let block = rng.random::<[u8; BLOCK_SIZE]>().to_vec();
            let (res, _) = file.write_all_at(block, pos).await;
            res?;
            pos += BLOCK_SIZE as u64;
        }
        println!("Case 1. Push bytes:\t{:?}", tm.elapsed());

        // Case 2. Iterate blocks
        let tm = std::time::Instant::now();
        let block = vec![0u8; BLOCK_SIZE];
        let mut pos = 0;
        for _ in 0..COUNT {
            let (res, _block) = file.read_exact_at(block.clone(), pos).await;
            res?;
            pos += BLOCK_SIZE as u64;
        }
        println!("Case 2. Iterate blocks:\t{:?}", tm.elapsed());

        // Case 3. Random read
        let tm = std::time::Instant::now();
        let block = vec![0u8; BLOCK_SIZE];
        for _ in 0..COUNT {
            let pos = rng.random_range(0..COUNT) as u64;
            let (res, _block) = file.read_exact_at(block.clone(), pos).await;
            res?;
        }
        println!("Case 3. Random read:\t{:?}", tm.elapsed());

        // Case 4. Random update
        let tm = std::time::Instant::now();
        for _ in 0..COUNT {
            let block = rng.random::<[u8; BLOCK_SIZE]>().to_vec();
            let pos = rng.random_range(0..COUNT) as u64;
            let (res, _) = file.write_all_at(block, pos).await;
            res?;
        }
        println!("Case 4. Random update:\t{:?}", tm.elapsed());

        println!("");

        // Return
        Ok(())
    })
}


fn main() -> Result<(), std::io::Error> {
    test_std()?;
    test_std_ext()?;
    test_tokio()?;
    test_tokio_uring()?;
    Ok(())
}
