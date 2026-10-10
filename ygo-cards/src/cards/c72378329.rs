//! Beast-Eyes Pendulum Dragon
use super::prelude::*;

pub const ID: u32 = 72378329;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,s.ffilter,aux.FilterBoolFunctionEx(Card.IsRace,RACE_BEAST))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_BEAST, Some(fc), st, Some(tp)))),
        ],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit,nil,nil,nil,false)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            cannot_be_lizard: Some(false),
            ..Default::default()
        },
    );
    // damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    // e4:SetValue(s.valcheck)
    effect::set_value_fn(d, e4, lua_fn(valcheck));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,fc,sumtype,tp) and c:IsAttribute(ATTRIBUTE_DARK,fc,sumtype,tp)
    card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), sumtype, Some(tp)) && card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(fc), sumtype, Some(tp))
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return (st&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION
    let st = a.int(3) as u64;
    let _ = d;
    (st & SUMMON_TYPE_FUSION as u64) == SUMMON_TYPE_FUSION as u64
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetReleaseGroup(tp)
    duel::get_release_group(d, tp, false, false, None)
}
/// `s.contactop(g)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    // Duel.Release(g,REASON_COST|REASON_MATERIAL)
    duel::release(d, g.clone(), REASON_COST | REASON_MATERIAL, None);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsRelateToBattle() and bc:IsMonster()
    card::is_relate_to_battle(d, c) && card::is_monster(d, bc.nil())
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local dam=e:GetLabel()
    let dam = effect::get_label(d, a.e);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(1);
    // local g=c:GetMaterial():Filter(Card.IsRace,nil,RACE_BEAST,c,SUMMON_TYPE_FUSION)
    let mat = card::get_material(d, c);
    let g = group::filter(d, &mat, &|d, x| card::is_race_ctx(d, x, RACE_BEAST, Some(c), SUMMON_TYPE_FUSION as u64, None), NO_EX);
    // local atk=0
    let mut atk: i64 = 0;
    // if #g>0 then
    if !g.is_empty() {
        // atk=g:GetFirst():GetTextAttack()
        let first = group::get_first(d, &g).nil();
        atk = card::get_text_attack(d, first) as i64;
        // if atk<0 then atk=0 end
        if atk < 0 {
            atk = 0;
        }
    }
    // e:GetLabelObject():SetLabel(atk)
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, lo, &[atk]);
}
