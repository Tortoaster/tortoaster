export interface IProjectPreview {
  id: string;
  name: string;
  preview: string;
  thumbnailId: string;
  datePosted: Date;
}

export interface IProject {
  id: string;
  name: string;
  thumbnailId: string;
  projectUrl?: string;
  datePosted: Date;
}
