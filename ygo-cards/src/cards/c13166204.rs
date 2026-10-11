//! Gagagarush
//! ガガガラッシュ
use super::prelude::*;

pub const ID: u32 = 13166204;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GAGAGA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DAMAGE+CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BECOME_TARGET)
    effect::set_code(d, e1, EVENT_BECOME_TARGET);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and c:IsSetCard(SET_GAGAGA)
    card::is_faceup(d, c) && card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE) && card::is_set_card(d, c, &[SET_GAGAGA as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return rp~=tp and re:IsMonsterEffect() and eg:IsExists(s.filter,1,nil,tp) and Duel.IsChainDisablable(ev)
    a.rp != tp
        && effect::is_monster_effect(d, a.re.nil())
        && group::is_exists(d, &a.eg.clone().nil(), &|d, c| filter(d, c, tp), 1, NO_EX)
        && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,re:GetHandler(),1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(rc)), 1, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let re = a.re.nil();
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(re:GetHandler(),REASON_EFFECT)~=0 then
    if duel::negate_effect(d, a.ev as u8) {
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) && duel::destroy(d, CardSet::from_card(rc), REASON_EFFECT, None, None) != 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local a=re:GetHandler():GetAttack()
            let mut atk = card::get_attack(d, rc);
            // local b=re:GetHandler():GetDefense()
            let def = card::get_defense(d, rc);
            // if b>a then a=b end
            if def > atk {
                atk = def;
            }
            // if a<0 then a=0 end
            if atk < 0 {
                atk = 0;
            }
            // if a>0 then Duel.Damage(1-tp,a,REASON_EFFECT) end
            if atk > 0 {
                duel::damage(d, 1 - tp, atk as i64, REASON_EFFECT, false, None);
            }
        }
    }
}
