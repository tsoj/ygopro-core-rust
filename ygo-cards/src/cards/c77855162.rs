//! Tilting Entrainment
use super::prelude::*;

pub const ID: u32 = 77855162;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // If this card is Synchro Summoned: You can Special Summon 1 Level 4 or lower Pendulum Monster from your hand or face-up from your Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(synchro_cond)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // At the end of the Damage Step, when this card or your Pendulum Monster battles an opponent's monster, but the opponent's monster was not destroyed by the battle: You can destroy that opponent's monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon1)
    effect::set_condition(d, e2, Some(condition(descon1)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e2b=e2:Clone()
    let e2b = effect::clone(d, e2);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2b, LOCATION_MZONE);
    // e2b:SetCondition(s.descon2)
    effect::set_condition(d, e2b, Some(condition(descon2)));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
// function(e) return e:GetHandler():IsSynchroSummoned() end
fn synchro_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, c)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsType(TYPE_PENDULUM) and c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_type(d, c, TYPE_PENDULUM)
        && card::is_level_below(d, c, 4)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    // if c:IsLocation(LOCATION_HAND) then
    if card::is_location(d, c, LOCATION_HAND) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    } else {
        // return c:IsFaceup() and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        card::is_faceup(d, c) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_EXTRA) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.descon1(e,tp,eg,ep,ev,re,r,rp)
fn descon1(d: &mut Duel, a: &EffArgs) -> bool {
    // local a,b=Duel.GetBattleMonster(tp)
    let (ba, bb) = duel::get_battle_monster(d, a.tp);
    // e:SetLabelObject(b)
    effect::set_label_object(d, a.e, card_value(bb));
    // return a and b and b:IsRelateToBattle() and b:IsLocation(LOCATION_MZONE) and a:IsStatus(STATUS_OPPO_BATTLE)
    match (ba, bb) {
        (Some(x), Some(y)) => card::is_relate_to_battle(d, y) && card::is_location(d, y, LOCATION_MZONE) && card::is_status(d, x, STATUS_OPPO_BATTLE),
        _ => false,
    }
}
// function s.descon2(e,tp,eg,ep,ev,re,r,rp)
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local a,b=Duel.GetBattleMonster(tp)
    let (ba, bb) = duel::get_battle_monster(d, a.tp);
    // e:SetLabelObject(b)
    effect::set_label_object(d, a.e, card_value(bb));
    // return a and b and b:IsRelateToBattle() and b:IsLocation(LOCATION_MZONE)
    //     and a:IsStatus(STATUS_OPPO_BATTLE) and a:IsFaceup() and a:IsType(TYPE_PENDULUM)
    match (ba, bb) {
        (Some(x), Some(y)) => {
            card::is_relate_to_battle(d, y)
                && card::is_location(d, y, LOCATION_MZONE)
                && card::is_status(d, x, STATUS_OPPO_BATTLE)
                && card::is_faceup(d, x)
                && card::is_type(d, x, TYPE_PENDULUM)
        }
        _ => false,
    }
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetLabelObject(),1,tp,0)
    let tc = effect::get_label_object(d, a.e).as_card().map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc, 1, a.tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if tc and tc:IsRelateToBattle() and tc:IsControler(1-tp) then
    if let Some(tc) = tc {
        if card::is_relate_to_battle(d, tc) && card::is_controler(d, tc, 1 - tp) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// A Lua card-or-nil value for `SetLabelObject`.
fn card_value(c: Option<CardId>) -> Value {
    match c {
        Some(c) => Value::Card(c),
        None => Value::Nil,
    }
}
