    let gateway = Sqlite3Gateway { connection: RefCell::new(Some(conn)) };
    let repos = BlogRepository { gateway: gateway };

    let query = select::<Post>().from::<Posts>();

    let e1 = query.execute(&repos).for_each(|post| {
        println!("{:?}", post);
        future::ready(())
    });

    await!(e1);