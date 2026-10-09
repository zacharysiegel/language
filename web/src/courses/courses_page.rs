use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use shared::artifact::CourseIndexSource;
use shared::course;

use crate::course_data::{self, course_index_gate};

#[component]
pub fn CoursesPage() -> impl IntoView {
    course_index_gate(|course_index: Arc<CourseIndexSource>| {
        let course_summaries: Vec<(String, String, String)> = course_index
            .course_index
            .courses
            .iter()
            .map(|(course_id, index_entry)| {
                let languages: String = format!(
                    "{} for {} speakers",
                    course::language_name(&index_entry.target_language),
                    course::language_name(&index_entry.source_language),
                );

                (course_data::course_path(course_id), index_entry.title.clone(), languages)
            })
            .collect();
        let course_count_caption: String = match course_summaries.len() {
            1 => "1 course".to_string(),
            count => format!("{count} courses"),
        };
        let course_items: Vec<_> = course_summaries
            .into_iter()
            .map(|(href, title, languages)| {
                view! {
                    <li>
                        <A href=href>
                            <span class="course-name">{title}</span>
                            <span class="course-languages">{languages}</span>
                        </A>
                    </li>
                }
            })
            .collect();

        view! {
            <div class="course-title">
                <h1>"Courses"</h1>
                <span class="page-caption">{course_count_caption}</span>
            </div>
            <ul class="courses">{course_items}</ul>
        }
    })
}
