//! Great Mammoth of the Netherworld
use super::prelude::*;

pub const ID: u32 = 80461466;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CALL_OF_THE_HAUNTED, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is Special Summoned and you control "Call of the Haunted" or a Zombie monster, except "Great Mammoth of the Netherworld": You can target 1 card on the field; destroy it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY | EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // A Zombie Xyz Monster that has this card as material gains this effect: ● Gains 1000 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_XMATERIAL)
    effect::set_type(d, e2, EFFECT_TYPE_XMATERIAL);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e) local c=e:GetHandler() return c:IsRace(RACE_ZOMBIE) and c:IsXyzMonster() end)
    effect::set_condition(
        d,
        e2,
        Some(lua_fn(|d, a: Args| {
            let c = effect::get_handler(d, a.e(0)).nil();
            card::is_race(d, c, RACE_ZOMBIE as u64) && card::is_xyz_monster(d, c)
        })),
    );
    // e2:SetValue(1000)
    effect::set_value(d, e2, 1000);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.desconfilter(c)`
fn desconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsCode(CARD_CALL_OF_THE_HAUNTED) or (c:IsLocation(LOCATION_MZONE) and c:IsRace(RACE_ZOMBIE) and not c:IsCode(id)))
    card::is_faceup(d, c)
        && (card::is_code(d, c, &[CARD_CALL_OF_THE_HAUNTED])
            || (card::is_location(d, c, LOCATION_MZONE) && card::is_race(d, c, RACE_ZOMBIE as u64) && !card::is_code(d, c, &[ID])))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    //     and Duel.IsExistingMatchingCard(s.desconfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&desconfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
