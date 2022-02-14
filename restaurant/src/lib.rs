mod front_of_house { // mod keyword 를 이용하여 module의 이름을 지정 및 정의
    mod hosting { // module내 다른 module을 정의할 수 있ㅇ므
        fn add_to_waitlist() {} // module에는 sturct, enums, constants, traits, functions 를 추가 가능

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}