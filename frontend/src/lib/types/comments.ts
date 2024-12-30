export interface ICommentWithUser {
    id: number,
    userId: string,
    name?: string,
    isAdmin: boolean,
    message: string,
    datePosted: Date,
}
