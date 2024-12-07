// 导入所需的库和模块
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

// 定义 MovieAccountState 结构体，表示影评账户的状态
#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieAccountState {
    pub discriminator: String, // 鉴别器字段，用于区分不同类型的账户
    pub is_initialized: bool,  // 是否已初始化
    pub reviewer: Pubkey,      // 影评者的公钥
    pub rating: u8,            // 评分
    pub title: String,         // 影片标题
    pub description: String,   // 影片描述
}

// MovieAccountState 结构体的实现块
impl MovieAccountState {
    pub const DISCRIMINATOR: &'static str = "review"; // 鉴别器常量，标识为"review"

    // 计算账户的大小，以便初始化
    pub fn get_account_size(title: String, description: String) -> usize {
        (4 + MovieAccountState::DISCRIMINATOR.len())
            + 1
            + 1
            + (4 + title.len())
            + (4 + description.len())
    }
}

// 定义 MovieCommentCounter 结构体，表示评论计数器账户的状态
#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieCommentCounter {
    pub discriminator: String, // 鉴别器字段，用于区分不同类型的账户
    pub is_initialized: bool,  // 是否已初始化
    pub counter: u64,          // 评论数量计数器
}

// MovieCommentCounter 结构体的实现块
impl MovieCommentCounter {
    pub const DISCRIMINATOR: &'static str = "counter"; // 鉴别器常量，标识为"counter"
    pub const SIZE: usize = (4 + MovieCommentCounter::DISCRIMINATOR.len()) + 1 + 8; // 计算账户的大小
}

// 定义 MovieComment 结构体，表示评论账户的状态
#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieComment {
    pub discriminator: String, // 鉴别器字段，用于区分不同类型的账户
    pub is_initialized: bool,  // 是否已初始化
    pub review: Pubkey,        // 关联的影评的公钥
    pub commenter: Pubkey,     // 评论者的公钥
    pub comment: String,       // 评论内容
    pub count: u64,            // 评论计数器
}

// MovieComment 结构体的实现块
impl MovieComment {
    pub const DISCRIMINATOR: &'static str = "comment"; // 鉴别器常量，标识为"comment"

    // 计算账户的大小，以便初始化
    pub fn get_account_size(comment: String) -> usize {
        (4 + MovieComment::DISCRIMINATOR.len()) + 1 + 32 + 32 + (4 + comment.len()) + 8
    }
}

// 下面是对 IsInitialized 特征的实现
impl Sealed for MovieAccountState {}

impl IsInitialized for MovieAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for MovieCommentCounter {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for MovieComment {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
