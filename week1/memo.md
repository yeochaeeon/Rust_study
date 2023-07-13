<스터디 1일차>

과제 {rps. trianle. rps2. shooting} Review 

[과제 리뷰 발표]
i32, u32 차이는 값의 부호(+,-) 여부
둘이 가지고 있는 바이트는 같음

python에서 split은 whitespace로 씀

let result를 선언해서 result에 if 문 자체를 넣으면 코드를 효율적으로 짤 수 있음

거듭제곱은 pow(2) 이런 식으로 쓰니까 잘 되더라~ : ^이나 **을 쓰면 거듭제곱

[..2] 이게 range 개념
vec가 우리의 list랑 같아요

input.split할 때 whitespace 안 쓰고 수동으로 작동하게 하려면
입력받은 변수를 가변 list에 for문으로 push 해주면 됨

\r 과 \n이 무엇이냐
r은 시작이냐 묻는거고 n은 다음 줄로 넘어가냐 이건데
rps input에서는 띄어쓰기만 쓰기 때문에 n은 필요없다

trim이 뭐예요?
: 입력을 하다가 처음과 끝에 공백을 넣었어. 그럼 그 공백을 지워주는 역할

collect::VEC<_>  
-> 앞에서 whitespace 나누고 parse한 것들을 collect해서 vec로 집어넣는다

split_whitespace 보다 split_ascii_whitespace가 더 가볍게 돌아가요

cmp 앞 뒤 값 두 개 비교하는거고
ordering::Equal 뭐 이런식으로 비교합니다
assembly 한거라고 하시는데 무슨 말씀이시죠

rps2에서
use std::collections::HashMap에서 hashmap 불러와야하고
dictionary를 hashmap을 통해 쓴다
insert는 dictionary에 넣는거, get은 요소를 꺼내오는 거

for &i in &l에서 &가 무슨 뜻인가? -> ?

shooting에 h o h o 이런식으로 쓴다
python과는 달리 rust에서는
for c in shoot.chars() 이런식으로 문자열을 꺼내오는 방식으로 해야됨
이게 아니라 다른 방법으로는
split으로 하나씩 구분하고 map으로 빼올 수 있음

bool 은 true false 상태만 저장하는거
dictionary 말고 쓰려면 bool로 r p s 세 개의 변수를 false로 지정하고 시작해도 됨
character랑 string 차이가 뭐임? ->

iterator: 안에 있는 요소에 각각 접속할 때 값만 들고 올 수 있음(비교할때는 iterator, 변수를 바꿔야 할 때는 반대) : tuple이라고 생각하면 됨
I앞에 *써서 포인터 개념으로 주소를 가져와야 함
주소를 갖고 오면 주소가 가리키는 방향에 있는 값도 같이 바뀌는 거야
긍까 비교할때만 iterator을 써야함요
clone으로 복제해서 바꾸는 방법도 있지요

python에 set 개념이 rust에서는 hastset이 있음 그거 쓰면 됨

t를 trim하고 chars()에 vec으로 collect를 한거랑
t.chars()를 바로 쓰는 거랑 차이 안나 (for에서)

hast_set을 쓸거면 cloned을 쓰래요
set처리하고 vec로 바꾼다음에 조건문을 써야함
not in은 !contains()

&는 빌림 포인터: 값만 빌려온다라고 생각하면 될 듯
변수를 만들면 주소가 보통 할당이 돼, 어느 주소에 어떤 값을 넣을거야? 이런 변수 주소에 값 넣을거야 하는 게 &mut
&는 주소를 가져올 때도 쓰임

input.trim().split(“”).collect(); 
걍 문자 하나하나를 분할 해주는 거 

C에서 배열에 hello를 저장한다 할 때, 한 칸 한칸에 h,e,l,.. 따로따로 저장을 하는데
Vec<&str>하면 split한 걸 각각 칸에 저장한다는 느낌이고
Vec<String>은 문자열 자체를 한 칸에 박는다 이 느낌인데 
어차피 split으로 분할하고 넣으면 어차피 똑같아져요

Vec<char> 로 쓰면 나중에 한 번 더 변환할 필요 없어요
