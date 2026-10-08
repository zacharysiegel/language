use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use shared::course;

use crate::course_data::{self, LoadedVersion, version_gate};

#[component]
pub fn CoursesPage() -> impl IntoView {
    version_gate(|version: Arc<LoadedVersion>| {
        let course_summaries: Vec<(String, String, String)> = version
            .source
            .manifest
            .courses
            .iter()
            .map(|(course_id, manifest_course)| {
                let languages: String = format!(
                    "{} for {} speakers",
                    course::language_name(&manifest_course.target_language),
                    course::language_name(&manifest_course.source_language),
                );

                (course_data::course_path(course_id), manifest_course.title.clone(), languages)
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
