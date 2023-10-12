/* tslint:disable */
/* eslint-disable */
/**
 * Nine Yards REST API
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: rabbitminers@zohomail.eu
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
import type { SubTask } from './SubTask';
import {
    SubTaskFromJSON,
    SubTaskFromJSONTyped,
    SubTaskToJSON,
} from './SubTask';
import type { Task } from './Task';
import {
    TaskFromJSON,
    TaskFromJSONTyped,
    TaskToJSON,
} from './Task';

/**
 * A struct containing the full task and all of its
 * sub-tasks used in most endpoints where tasks are
 * fetched
 * @export
 * @interface FullTask
 */
export interface FullTask {
    /**
     * 
     * @type {Array<SubTask>}
     * @memberof FullTask
     */
    subTasks: Array<SubTask>;
    /**
     * 
     * @type {Task}
     * @memberof FullTask
     */
    task: Task;
}

/**
 * Check if a given object implements the FullTask interface.
 */
export function instanceOfFullTask(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "subTasks" in value;
    isInstance = isInstance && "task" in value;

    return isInstance;
}

export function FullTaskFromJSON(json: any): FullTask {
    return FullTaskFromJSONTyped(json, false);
}

export function FullTaskFromJSONTyped(json: any, ignoreDiscriminator: boolean): FullTask {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'subTasks': ((json['sub_tasks'] as Array<any>).map(SubTaskFromJSON)),
        'task': TaskFromJSON(json['task']),
    };
}

export function FullTaskToJSON(value?: FullTask | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'sub_tasks': ((value.subTasks as Array<any>).map(SubTaskToJSON)),
        'task': TaskToJSON(value.task),
    };
}
