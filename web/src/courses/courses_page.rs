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
        let course_items: Vec<_> = course_summaries
            .into_iter()
            .map(|(href, title, languages)| {
                view! {
                    <li>
                        <A href=href>
                            <span class="unit-title">{title}</span>
                            <span class="unit-status muted">{languages}</span>
                        </A>
                    </li>
                }
            })
            .collect();

        view! {
            <div class="course-title">
                <h1>"Courses"</h1>
                <span class="page-caption">{format!("{} courses", course_items.len())}</span>
            </div>
            <ul class="units">{course_items}</ul>
        }
    })
}
