pub mod applications;
pub mod members;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{
    extract,
    response::Response,
    routing::{delete, get, post, put},
    Extension, Router,
};
use sea_orm::DatabaseConnection;
use tower_sessions::Session;

use self::applications::group_application_routes;
use self::members::group_member_routes;

use crate::auth::data;
use crate::auth::data::groups::get_group_dto;
use crate::auth::model::groups::{NewGroupDto, UpdateGroupDto};
use crate::auth::permissions::require_permissions;

pub fn group_routes() -> Router {
    Router::new()
        .route("/", post(create_group))
        .route("/", get(get_groups))
        .route("/:group_id", get(get_group_by_id))
        .route("/:group_id", put(update_group))
        .route("/:group_id", delete(delete_group))
        .route("/:group_id/filters", get(get_group_filters))
        .nest("", group_member_routes())
        .nest("/applications", group_application_routes())
}

#[utoipa::path(
    post,
    path = "/groups",
    responses(
        (status = 200, description = "Created group info", body = GroupDto),
        (status = 403, description = "Insufficient permissions", body = String),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    security(
        ("login" = [])
    )
)]
pub async fn create_group(
    Extension(db): Extension<DatabaseConnection>,
    session: Session,
    extract::Json(payload): extract::Json<NewGroupDto>,
) -> Response {
    match require_permissions(&db, session).await {
        Ok(_) => (),
        Err(response) => return response,
    };

    match data::groups::create_group(&db, payload).await {
        Ok(group) => match get_group_dto(&db, Some(vec![group.id])).await {
            Ok(mut dto) => {
                if dto.is_empty() {
                    return (StatusCode::NOT_FOUND, "Group not found").into_response();
                }

                (StatusCode::OK, Json(dto.pop())).into_response()
            }
            Err(err) => {
                println!("{}", err);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error getting group info",
                )
                    .into_response()
            }
        },
        Err(err) => {
            if err.is::<sea_orm::error::DbErr>() {
                println!("{}", err);

                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error creating new group",
                )
                    .into_response();
            }

            (StatusCode::BAD_REQUEST, err.to_string()).into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/groups",
    responses(
        (status = 200, description = "List of groups", body = Vec<GroupDto>),
        (status = 403, description = "Insufficient permissions", body = String),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    security(
        ("login" = [])
    )
)]
pub async fn get_groups(
    Extension(db): Extension<DatabaseConnection>,
    session: Session,
) -> Response {
    match require_permissions(&db, session).await {
        Ok(_) => (),
        Err(response) => return response,
    };

    match data::groups::get_group_dto(&db, None).await {
        Ok(groups) => (StatusCode::OK, Json(groups)).into_response(),
        Err(err) => {
            println!("{}", err);

            (StatusCode::INTERNAL_SERVER_ERROR, "Error getting groups").into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/groups/{group_id}",
    responses(
        (status = 200, description = "Group info", body = GroupDto),
        (status = 403, description = "Insufficient permissions", body = String),
        (status = 404, description = "Not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    security(
        ("login" = [])
    )
)]
pub async fn get_group_by_id(
    Extension(db): Extension<DatabaseConnection>,
    session: Session,
    Path(group_id): Path<(i32,)>,
) -> Response {
    match require_permissions(&db, session).await {
        Ok(_) => (),
        Err(response) => return response,
    };

    match get_group_dto(&db, Some(vec![group_id.0])).await {
        Ok(mut group) => {
            if group.is_empty() {
                return (StatusCode::NOT_FOUND, "Group not found").into_response();
            }

            (StatusCode::OK, Json(group.pop())).into_response()
        }
        Err(err) => {
            println!("{}", err);

            (StatusCode::INTERNAL_SERVER_ERROR, "Error getting groups").into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/groups/{group_id}/filters",
    responses(
        (status = 200, description = "Group filters", body = Vec<GroupFiltersDto>),
        (status = 403, description = "Insufficient permissions", body = String),
        (status = 404, description = "Not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    security(
        ("login" = [])
    )
)]
pub async fn get_group_filters(
    Extension(db): Extension<DatabaseConnection>,
    session: Session,
    Path(group_id): Path<(i32,)>,
) -> Response {
    match require_permissions(&db, session).await {
        Ok(_) => (),
        Err(response) => return response,
    };

    match data::groups::filters::get_group_filters(&db, group_id.0).await {
        Ok(filters) => match filters {
            Some(filters) => (StatusCode::OK, Json(filters)).into_response(),
            None => (StatusCode::NOT_FOUND, "Group filters not found").into_response(),
        },
        Err(err) => {
            println!("{}", err);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting group filters",
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    put,
    path = "/groups/{group_id}",
    responses(
        (status = 200, description = "Updated group info", body = GroupDto),
        (status = 403, description = "Insufficient permissions", body = String),
        (status = 404, description = "Not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    security(
        ("login" = [])
    )
)]
pub async fn update_group(
    Extension(db): Extension<DatabaseConnection>,
    session: Session,
    Path(group_id): Path<(i32,)>,
    extract::Json(payload): extract::Json<UpdateGroupDto>,
) -> Response {
    match require_permissions(&db, session).await {
        Ok(_) => (),
        Err(response) => return response,
    };

    match data::groups::update_group(&db, group_id.0, payload).await {
        Ok(group) => match get_group_dto(&db, Some(vec![group.id])).await {
            Ok(mut dto) => {
                if dto.is_empty() {
                    return (StatusCode::NOT_FOUND, "Group not found").into_response();
                }

                (StatusCode::OK, Json(dto.pop())).into_response()
            }
            Err(err) => {
                println!("{}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error getting group info",
                )
                    .into_response()
            }
        },
        Err(err) => {
            let err_string = err.to_string();
            if err_string.contains("Filter rule with id")
                && err_string.contains("does not belong to group")
            {
                return (StatusCode::FORBIDDEN, err_string).into_response();
            }

            println!("{}", err);

            (StatusCode::INTERNAL_SERVER_ERROR, "Error updating group").into_response()
        }
    }
}

#[utoipa::path(
    delete,
    path = "/groups/{group_id}",
    responses(
        (status = 200, description = "Group deleted successfully", body = GroupDto),
        (status = 403, description = "Insufficient permissions", body = String),
        (status = 404, description = "Not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    security(
        ("login" = [])
    )
)]
pub async fn delete_group(
    Extension(db): Extension<DatabaseConnection>,
    session: Session,
    Path(group_id): Path<(i32,)>,
) -> Response {
    match require_permissions(&db, session).await {
        Ok(_) => (),
        Err(response) => return response,
    };

    match data::groups::delete_group(&db, group_id.0).await {
        Ok(result) => match result {
            Some(id) => (StatusCode::OK, format!("Deleted group with id {}", id)).into_response(),
            None => (StatusCode::NOT_FOUND, "Group not found").into_response(),
        },
        Err(err) => {
            println!("{}", err);

            (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting group").into_response()
        }
    }
}
