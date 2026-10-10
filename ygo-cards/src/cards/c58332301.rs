//! Evil HERO Dark Gaia
use super::prelude::*;

pub const ID: u32 = 58332301;

pub const INFO: ScriptInfo = ScriptInfo { dark_calling: true, listed_names: &[CARD_DARK_FUSION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion material
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_FIEND),aux.FilterBoolFunctionEx(Card.IsRace,RACE_ROCK))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_FIEND, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_ROCK, Some(fc), st, Some(tp)))),
        ],
    );
    // lizard check
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(CARD_CLOCK_LIZARD)
    effect::set_code(d, e0, CARD_CLOCK_LIZARD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCondition(s.lizcon)
    effect::set_condition(d, e0, Some(condition(lizcon)));
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.EvilHeroLimit)
    effect::set_value_fn(d, e1, aux::evil_hero_limit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change original ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change battle position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e3, CATEGORY_POSITION);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetTarget(s.postg)
    effect::set_target(d, e3, Some(cost_target(postg)));
    // e3:SetOperation(s.posop)
    effect::set_operation(d, e3, Some(operation(posop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.lizcon(e,tp,eg,ep,ev,re,r,rp)`
fn lizcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // return not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),EFFECT_SUPREME_CASTLE)
    let hp = effect::get_handler_player(d, a.e);
    duel::is_player_affected_by_effect(d, hp, EFFECT_SUPREME_CASTLE).is_none()
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local val=0
    let mut val: i64 = 0;
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local a=tc:GetAttack()
        let mut atk = card::get_attack(d, tc) as i64;
        // if a<0 then a=0 end
        if atk < 0 {
            atk = 0;
        }
        // val=val+a
        val += atk;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetValue(val)
    effect::set_value(d, e1, val);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDefensePos,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_defense_pos(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsDefensePos,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_defense_pos(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, 0, 0);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsDefensePos,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_defense_pos(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.ChangePosition(g,0,0,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK,true)
    duel::change_position(d, g, 0, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_ATTACK as u8), true);
}
