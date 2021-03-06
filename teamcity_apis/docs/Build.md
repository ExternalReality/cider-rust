# Build

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> |  | [optional]
**task_id** | Option<**i64**> |  | [optional]
**build_type_id** | Option<**String**> |  | [optional]
**build_type_internal_id** | Option<**String**> |  | [optional]
**number** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**state** | Option<**String**> |  | [optional]
**running** | Option<**bool**> |  | [optional]
**composite** | Option<**bool**> |  | [optional]
**failed_to_start** | Option<**bool**> |  | [optional]
**personal** | Option<**bool**> |  | [optional]
**percentage_complete** | Option<**i32**> |  | [optional]
**branch_name** | Option<**String**> |  | [optional]
**default_branch** | Option<**bool**> |  | [optional]
**unspecified_branch** | Option<**bool**> |  | [optional]
**history** | Option<**bool**> |  | [optional]
**pinned** | Option<**bool**> |  | [optional]
**href** | Option<**String**> |  | [optional]
**web_url** | Option<**String**> |  | [optional]
**queue_position** | Option<**i32**> |  | [optional]
**limited_changes_count** | Option<**i32**> |  | [optional]
**artifacts_directory** | Option<**String**> |  | [optional]
**links** | Option<[**crate::models::Links**](links.md)> |  | [optional]
**status_text** | Option<**String**> |  | [optional]
**build_type** | Option<[**crate::models::BuildType**](buildType.md)> |  | [optional]
**comment** | Option<[**crate::models::Comment**](comment.md)> |  | [optional]
**tags** | Option<[**crate::models::Tags**](tags.md)> |  | [optional]
**pin_info** | Option<[**crate::models::Comment**](comment.md)> |  | [optional]
**user** | Option<[**crate::models::User**](user.md)> |  | [optional]
**start_estimate** | Option<**String**> |  | [optional]
**wait_reason** | Option<**String**> |  | [optional]
**finish_estimate** | Option<**String**> |  | [optional]
**running_info** | Option<[**crate::models::ProgressInfo**](progress-info.md)> |  | [optional]
**canceled_info** | Option<[**crate::models::Comment**](comment.md)> |  | [optional]
**queued_date** | Option<**String**> |  | [optional]
**start_date** | Option<**String**> |  | [optional]
**finish_date** | Option<**String**> |  | [optional]
**triggered** | Option<[**crate::models::TriggeredBy**](TriggeredBy.md)> |  | [optional]
**last_changes** | Option<[**crate::models::Changes**](changes.md)> |  | [optional]
**changes** | Option<[**crate::models::Changes**](changes.md)> |  | [optional]
**revisions** | Option<[**crate::models::Revisions**](Revisions.md)> |  | [optional]
**versioned_settings_revision** | Option<[**crate::models::Revision**](Revision.md)> |  | [optional]
**artifact_dependency_changes** | Option<[**crate::models::BuildChanges**](buildChanges.md)> |  | [optional]
**agent** | Option<[**crate::models::Agent**](agent.md)> |  | [optional]
**compatible_agents** | Option<[**crate::models::Agents**](agents.md)> |  | [optional]
**test_occurrences** | Option<[**crate::models::TestOccurrences**](testOccurrences.md)> |  | [optional]
**problem_occurrences** | Option<[**crate::models::ProblemOccurrences**](problemOccurrences.md)> |  | [optional]
**artifacts** | Option<[**crate::models::Files**](files.md)> |  | [optional]
**related_issues** | Option<[**crate::models::IssuesUsages**](issuesUsages.md)> |  | [optional]
**properties** | Option<[**crate::models::Properties**](properties.md)> |  | [optional]
**resulting_properties** | Option<[**crate::models::Properties**](properties.md)> |  | [optional]
**attributes** | Option<[**crate::models::Entries**](entries.md)> |  | [optional]
**statistics** | Option<[**crate::models::Properties**](properties.md)> |  | [optional]
**metadata** | Option<[**crate::models::Datas**](datas.md)> |  | [optional]
**snapshot_dependencies** | Option<[**crate::models::Builds**](builds.md)> |  | [optional]
**artifact_dependencies** | Option<[**crate::models::Builds**](builds.md)> |  | [optional]
**custom_artifact_dependencies** | Option<[**crate::models::ArtifactDependencies**](artifact-dependencies.md)> |  | [optional]
**settings_hash** | Option<**String**> |  | [optional]
**current_settings_hash** | Option<**String**> |  | [optional]
**modification_id** | Option<**String**> |  | [optional]
**chain_modification_id** | Option<**String**> |  | [optional]
**replacement_ids** | Option<[**crate::models::Items**](items.md)> |  | [optional]
**related** | Option<[**crate::models::Related**](related.md)> |  | [optional]
**triggering_options** | Option<[**crate::models::BuildTriggeringOptions**](buildTriggeringOptions.md)> |  | [optional]
**used_by_other_builds** | Option<**bool**> |  | [optional]
**status_change_comment** | Option<[**crate::models::Comment**](comment.md)> |  | [optional]
**vcs_labels** | Option<[**Vec<crate::models::VcsLabel>**](vcsLabel.md)> |  | [optional]
**detached_from_agent** | Option<**bool**> |  | [optional]
**finish_on_agent_date** | Option<**String**> |  | [optional]
**customized** | Option<**bool**> |  | [optional]
**customization** | Option<[**crate::models::Customizations**](customizations.md)> |  | [optional]
**locator** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


