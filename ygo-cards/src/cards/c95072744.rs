//! Tohushaddoll Grysta
//! 影雄の烬 エグリスタ
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 95072744;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply the effect of 1 non-Rock "Shaddoll" monster in your GY that activates when it is flipped face-up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.applytg)
    effect::set_target(d, e1, Some(cost_target(applytg)));
    // e1:SetOperation(s.applyop)
    effect::set_operation(d, e1, Some(operation(applyop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Fusion Summon 1 "Shaddoll" Fusion Monster from your Extra Deck, by banishing its materials from your hand, field, and/or GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsReason(REASON_EFFECT) and not Duel.IsPhase(PHASE_DAMAGE) end)
    effect::set_condition(d, e2, Some(condition(fusion_cond)));
    // e2:SetCost(s.fusioncost)
    effect::set_cost(d, e2, Some(cost_target(fusioncost)));
    // e2:SetTarget(Fusion.SummonEffTG(fusion_params))
    let tg = fusion::summon_eff_tg(d, fusion_params());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(fusion_params))
    let op = fusion::summon_eff_op(d, fusion_params());
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return not c:IsSummonLocation(LOCATION_EXTRA) or c:IsSetCard(SET_SHADDOLL) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(activity_counter));
}
// the fusion_params table of s.initial_effect
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        // fusfilter=function(c) return c:IsSetCard(SET_SHADDOLL) end
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_SHADDOLL as u16]))),
        // matfilter=Card.IsAbleToRemove
        matfilter: Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, tp: u8, chk: i64| card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32)))),
        // extrafil=s.fextra
        extrafil: Some(std::sync::Arc::new(|d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet| fextra(d, tp))),
        // extraop=Fusion.BanishMaterial
        extraop: Some(fusion::banish_material()),
        // extratg=s.extratg
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    }
}
// function s.fextra(e,tp,mg)
fn fextra(d: &mut Duel, tp: u8) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)),
            tp,
            LOCATION_GRAVE,
            0,
            NO_EX,
        );
        return (Some(g), None);
    }
    // return nil
    (None, None)
}
// function s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
// function(e) return e:GetHandler():IsReason(REASON_EFFECT) and not Duel.IsPhase(PHASE_DAMAGE) end
fn fusion_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_EFFECT) && !duel::is_phase(d, PHASE_DAMAGE)
}
// function(c) return not c:IsSummonLocation(LOCATION_EXTRA) or c:IsSetCard(SET_SHADDOLL) end
fn activity_counter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    !card::is_summon_location(d, c, LOCATION_EXTRA) || card::is_set_card(d, c, &[SET_SHADDOLL as u16])
}
// function s.applyfilter(c,e,tp)
fn applyfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsCode(id) then return false end
    if card::is_code(d, c, &[ID]) {
        return false;
    }
    // if not (not c:IsRace(RACE_ROCK) and c:IsSetCard(SET_SHADDOLL) and c:IsMonster()) then return false end
    if !(!card::is_race(d, c, RACE_ROCK) && card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_monster(d, c)) {
        return false;
    }
    // local effs={c:GetOwnEffects()}
    let effs = card::get_own_effects(d, c);
    // for _,eff in ipairs(effs) do
    for eff in effs {
        // if eff:GetType()&EFFECT_TYPE_FLIP>0 or eff:GetCode()==EVENT_FLIP then
        if effect::get_type(d, eff) & EFFECT_TYPE_FLIP > 0 || effect::get_code(d, eff) == EVENT_FLIP {
            // local tg=eff:GetTarget()
            let tg = effect::get_target(d, eff);
            // if tg==nil or tg(eff,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,0) then
            let ok = match tg {
                None => true,
                Some(tg) => tg
                    .call(
                        d,
                        &[
                            Value::Effect(eff),
                            Value::Int(tp as i64),
                            Value::Group(CardSet::new()),
                            Value::Int(PLAYER_NONE as i64),
                            Value::Int(0),
                            Value::Effect(e),
                            Value::Int(REASON_EFFECT as i64),
                            Value::Int(PLAYER_NONE as i64),
                            Value::Int(0),
                        ],
                    )
                    .truthy(),
            };
            if ok {
                return true;
            }
        }
    }
    // return false
    false
}
// function s.applytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn applytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.applyfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| applyfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.applyfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&|d, c| applyfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local available_effs={}
    let mut available_effs: Vec<EffectId> = Vec::new();
    // local effs={tc:GetOwnEffects()}
    let effs = card::get_own_effects(d, tc);
    // for _,eff in ipairs(effs) do
    for eff in effs {
        // if eff:GetType()&EFFECT_TYPE_FLIP>0 or eff:GetCode()==EVENT_FLIP then
        if effect::get_type(d, eff) & EFFECT_TYPE_FLIP > 0 || effect::get_code(d, eff) == EVENT_FLIP {
            // local tg=eff:GetTarget()
            let tg = effect::get_target(d, eff);
            // if tg==nil or tg(eff,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,0) then
            let ok = match tg {
                None => true,
                Some(tg) => tg
                    .call(
                        d,
                        &[
                            Value::Effect(eff),
                            Value::Int(a.tp as i64),
                            Value::Group(CardSet::new()),
                            Value::Int(PLAYER_NONE as i64),
                            Value::Int(0),
                            Value::Effect(a.e),
                            Value::Int(REASON_EFFECT as i64),
                            Value::Int(PLAYER_NONE as i64),
                            Value::Int(0),
                        ],
                    )
                    .truthy(),
            };
            if ok {
                // table.insert(available_effs,eff)
                available_effs.push(eff);
            }
        }
    }
    // local eff=nil
    // if #available_effs>1 then
    let eff = if available_effs.len() > 1 {
        // local available_effs_desc={}
        // for _,eff in ipairs(available_effs) do table.insert(available_effs_desc,eff:GetDescription()) end
        let descs: Vec<u64> = available_effs.iter().map(|&x| effect::get_description(d, x)).collect();
        // local op=Duel.SelectOption(tp,table.unpack(available_effs_desc))
        let op = duel::select_option(d, a.tp, true, &descs);
        // eff=available_effs[op+1]
        Some(available_effs[op as usize])
    } else {
        // eff=available_effs[1]
        available_effs.first().copied()
    };
    let eff = eff.nil();
    // Duel.Hint(HINT_OPSELECTED,1-tp,eff:GetDescription())
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, effect::get_description(d, eff));
    // Duel.ClearTargetCard()
    duel::clear_target_card(d);
    // tc:CreateEffectRelation(e)
    card::create_effect_relation(d, tc, a.e);
    // e:SetLabel(eff:GetLabel())
    let label = effect::get_labels(d, eff);
    effect::set_label(d, a.e, &label);
    // e:SetLabelObject(eff:GetLabelObject())
    let lobj = effect::get_label_object(d, eff);
    effect::set_label_object(d, a.e, lobj);
    // local tg=eff:GetTarget()
    // if tg then
    if let Some(tg) = effect::get_target(d, eff) {
        // tg(e,tp,eg,ep,ev,re,r,rp,1)
        let eg = a.eg.clone().map(Value::Group).unwrap_or(Value::Nil);
        let re = a.re.map(Value::Effect).unwrap_or(Value::Nil);
        tg.call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                eg,
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                re,
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
                Value::Int(1),
            ],
        );
        // eff:SetLabel(e:GetLabel())
        let label = effect::get_labels(d, a.e);
        effect::set_label(d, eff, &label);
        // eff:SetLabelObject(e:GetLabelObject())
        let lobj = effect::get_label_object(d, a.e);
        effect::set_label_object(d, eff, lobj);
    }
    // e:SetLabelObject(eff)
    effect::set_label_object(d, a.e, Value::Effect(eff));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
// function s.applyop(e,tp,eg,ep,ev,re,r,rp)
fn applyop(d: &mut Duel, a: &EffArgs) {
    // local eff=e:GetLabelObject()
    let eff = effect::get_label_object(d, a.e).as_effect();
    // if not (eff and eff:GetHandler():IsRelateToEffect(e)) then return end
    let Some(eff) = eff else { return };
    let h = effect::get_handler(d, eff).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // e:SetLabel(eff:GetLabel())
    let label = effect::get_labels(d, eff);
    effect::set_label(d, a.e, &label);
    // e:SetLabelObject(eff:GetLabelObject())
    let lobj = effect::get_label_object(d, eff);
    effect::set_label_object(d, a.e, lobj);
    // local op=eff:GetOperation()
    // if op then
    if let Some(op) = effect::get_operation(d, eff) {
        // op(e,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE)
        op.call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                Value::Group(CardSet::new()),
                Value::Int(PLAYER_NONE as i64),
                Value::Int(0),
                Value::Effect(a.e),
                Value::Int(REASON_EFFECT as i64),
                Value::Int(PLAYER_NONE as i64),
            ],
        );
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // e:SetLabelObject(nil)
    effect::set_label_object(d, a.e, Value::Nil);
}
// function s.fusioncost(e,tp,eg,ep,ev,re,r,rp,chk)
fn fusioncost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsSetCard(SET_SHADDOLL) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_set_card(d, c, &[SET_SHADDOLL as u16]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
