mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    mod cooking {
        fn select_ingredients() {}

        fn cut_vegetables() {}
    }

    mod preparing {
        fn prepare_dish() {}
    }

    mod cleaning {
        fn clean_dishes() {}

        fn clean_floor() {}
    }
}
