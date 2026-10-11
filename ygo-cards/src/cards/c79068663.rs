//! Junk Barrage
use super::prelude::*;

pub const ID: u32 = 79068663;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local eqc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let eqc = card::get_equip_target(d, h);
    // local des=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let des = group::get_first(d, &eg).nil();
    // return des:IsLocation(LOCATION_GRAVE) and des:GetReasonCard()==eqc and des:IsMonster()
    card::is_location(d, des, LOCATION_GRAVE) && card::get_reason_card(d, des) == eqc && card::is_monster(d, des)
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // eg:GetFirst():CreateEffectRelation(e)
    let eg = a.eg.clone().nil();
    let des = group::get_first(d, &eg).nil();
    card::create_effect_relation(d, des, a.e);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 0);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // local des=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let des = group::get_first(d, &eg).nil();
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // if des:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, des, a.e) {
        // local dam=des:GetAttack()/2
        let mut dam = card::get_attack(d, des) as f64 / 2.0;
        // if dam<0 then dam=0 end
        if dam < 0.0 {
            dam = 0.0;
        }
        // Duel.Damage(p,dam,REASON_EFFECT)
        duel::damage(d, p, lua_int(dam), REASON_EFFECT, false, None);
    }
}
