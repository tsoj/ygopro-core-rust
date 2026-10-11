//! Division
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 42427230;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, ID + 1], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon "Machine Tokens"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.tkncost)
    effect::set_cost(d, e1, Some(cost_target(tkncost)));
    // e1:SetTarget(s.tkntg)
    effect::set_target(d, e1, Some(cost_target(tkntg)));
    // e1:SetOperation(s.tknop)
    effect::set_operation(d, e1, Some(operation(tknop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy any number of Tokens you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tkncost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tkncost(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,id),tp,LOCATION_MZONE,0,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[ID])), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g==g:FilterCount(Card.IsReleasable,nil) and Duel.GetMZoneCount(tp,g)>0 end
    if a.chk == Some(0) {
        let releasable = group::filter_count(d, &g, &|d, c| card::is_releasable(d, c, None, None), NO_EX);
        return g.len() == releasable && duel::get_m_zone_count(d, a.tp, (&g).into(), None, None, None).0 > 0;
    }
    // g:Filter(Card.IsReleasable,nil)
    g = group::filter(d, &g, &|d, c| card::is_releasable(d, c, None, None), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g.clone(), REASON_COST, None);
    // e:SetLabel(#g*2)
    effect::set_label(d, a.e, &[(g.len() * 2) as i64]);
    true
}
/// `s.tkntg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tkntg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,200,200,1,RACE_MACHINE,ATTRIBUTE_FIRE) end
    if a.chk == Some(0) {
        return duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// The Machine Token (`Duel.IsPlayerCanSpecialSummonMonster` data).
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(200),
        defense: Some(200),
        level: Some(1),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_FIRE),
    }
}
/// `s.tknop(e,tp,eg,ep,ev,re,r,rp)`
fn tknop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,200,200,1,RACE_MACHINE,ATTRIBUTE_FIRE) then return end
    if !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0) {
        return;
    }
    // local ft=math.min(e:GetLabel(),Duel.GetLocationCount(tp,LOCATION_MZONE))
    let label = effect::get_label(d, a.e);
    let zones = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    let mut ft = label.min(zones as i64);
    // if ft==0 then return end
    if ft == 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=Duel.AnnounceNumberRange(tp,1,ft)
    let ct = duel::announce_number_range(d, a.tp, Some(1), Some(ft as u32), &[]).nil().0;
    // for i=1,ct do
    for _ in 0..ct {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // if Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // Inflict 500 damage when the tokens are destroyed
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EVENT_DESTROYED)
            effect::set_code(d, e1, EVENT_DESTROYED);
            // e1:SetOperation(function(e) Duel.Damage(1-tp,500,REASON_EFFECT) e:Reset() end)
            let tp = a.tp;
            effect::set_operation(
                d,
                e1,
                Some(lua_fn(move |d, a| {
                    duel::damage(d, 1 - tp, 500, REASON_EFFECT, false, None);
                    effect::reset(d, a.e(0));
                    Value::Nil
                })),
            );
            // token:RegisterEffect(e1,true)
            card::register_effect(d, token, e1, true);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and chkc:IsType(TYPE_TOKEN) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, a.tp) && card::is_type(d, chkc, TYPE_TOKEN);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsType,tp,LOCATION_ONFIELD,0,1,nil,TYPE_TOKEN) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_type(d, c, TYPE_TOKEN)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // local ct=Duel.GetMatchingGroupCount(Card.IsType,tp,LOCATION_ONFIELD,0,nil,TYPE_TOKEN)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_type(d, c, TYPE_TOKEN)), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsType,tp,LOCATION_ONFIELD,0,1,ct,nil,TYPE_TOKEN)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_type(d, c, TYPE_TOKEN)), a.tp, LOCATION_ONFIELD, 0, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
