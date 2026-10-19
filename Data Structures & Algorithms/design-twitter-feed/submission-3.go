
import "slices"

type Twitter struct {
    users map[int]*User
    clock int
}

type User struct {
    userId    int
    posts     []Post
    followers []int // Actually the users this user follows
}

type Post struct {
    tweetId   int
    timestamp int
}

func Constructor() Twitter {
    return Twitter{users: make(map[int]*User)}
}

func (this *Twitter) GetUser(userId int) *User {
    if this.users[userId] == nil {
        this.users[userId] = &User{userId: userId}
    }
    return this.users[userId]
}

func (this *Twitter) PostTweet(userId int, tweetId int) {
    this.clock++
    user := this.GetUser(userId)
    user.posts = append(user.posts, Post{
        tweetId:   tweetId,
        timestamp: this.clock,
    })
}

func (this *Twitter) GetNewsFeed(userId int) []int {
    user := this.GetUser(userId)

    posts := append([]Post(nil), user.posts...)

    for _, id := range user.followers {
        posts = append(posts, this.GetUser(id).posts...)
    }

    slices.SortFunc(posts, func(a, b Post) int {
        return b.timestamp - a.timestamp
    })

    limit := min(len(posts), 10)
    ans := make([]int, limit)

    for i := 0; i < limit; i++ {
        ans[i] = posts[i].tweetId
    }

    return ans
}

func (this *Twitter) Follow(followerId int, followeeId int) {
    if followerId == followeeId {
        return
    }

    follower := this.GetUser(followerId)
    this.GetUser(followeeId)

    for _, id := range follower.followers {
        if id == followeeId {
            return
        }
    }

    follower.followers = append(follower.followers, followeeId)
}

func (this *Twitter) Unfollow(followerId int, followeeId int) {
    follower := this.GetUser(followerId)

    follower.followers = slices.DeleteFunc(
        follower.followers,
        func(id int) bool {
            return id == followeeId
        },
    )
}
