## rust 2주차 

* usize 양의 정수타입
  - index의 음수참조를 안해줘서 .. list에선 usize써줘야함
  
  - iter.position = python으로 치면 find
  - 0..3 = python의 range
  - ans.push("땡") 답에 땡 집어넣기

* me
  - next() = input을 처음에 아무것도 안받는데 그래서 next로 바꿔준다?
  
  - iter().enumerate()
  
  - &firstday = & <- 빌림 연산자(참조)
  
  - iter().take() = 가져오기 ? list에서 가져오기

* 해중
  - (가리키는값) 그리고  &(주소값)
  - firstday.trim() ==day
  - &firstday.trim() == day

* 의찬
  - i32 option = none일수도 있따?를 알려줌?
  - d.clone() ; a=b해버리면 나중에 a를 바꾸면 b도 바꿔지니까
  - 그냥 clone써서 해결함

* 유성

  - 얕은복사
  - 깊은복사
  - 참조자(소유권x)

rust 책을 빌려서 좀 공부해야겠는데?

* 진목
  - std :: collections :: VecDeque
  - pop()맨뒤만 되잖아?
  - pop_front() + vecDeque 맨뒤꺼 빼서 앞에 넣는거
  - date.push_back(date[0])
  - date.pop_front()
    
  buf: 먼가 추천 해줌?
