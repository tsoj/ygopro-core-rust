//! Fire Lake of the Burning Abyss
use super::prelude::*;

pub const ID: u32 = 36006208;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BURNING_ABYSS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BURNING_ABYSS) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BURNING_ABYSS as u16]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.costfilter(c,rg,dg)`
fn costfilter(d: &mut Duel, c: CardId, rg: &CardSet, dg: &CardSet) -> bool {
    // local a=0
    let mut a: i64 = 0;
    // if dg:IsContains(c) then a=1 end
    if dg.contains(c) {
        a = 1;
    }
    // if c:GetEquipCount()==0 then return rg:IsExists(s.costfilter2,1,c,a,dg) end
    if card::get_equip_count(d, c) == 0 {
        return group::is_exists(d, rg, &|d, x| costfilter2(d, x, a, dg), 1, c.into());
    }
    // local eg=c:GetEquipGroup()
    let eg = card::get_equip_group(d, c);
    // for tc in aux.Next(eg) do if dg:IsContains(tc) then a=a+1 end end
    for tc in group::iter(d, &eg) {
        if dg.contains(tc) {
            a += 1;
        }
    }
    // return rg:IsExists(s.costfilter2,1,c,a,dg)
    group::is_exists(d, rg, &|d, x| costfilter2(d, x, a, dg), 1, c.into())
}
/// `s.costfilter2(c,a,dg)`
fn costfilter2(d: &mut Duel, c: CardId, mut a: i64, dg: &CardSet) -> bool {
    // if dg:IsContains(c) then a=a+1 end
    if dg.contains(c) {
        a += 1;
    }
    // if c:GetEquipCount()==0 then return #dg-a>=1 end
    if card::get_equip_count(d, c) == 0 {
        return dg.len() as i64 - a >= 1;
    }
    // local eg=c:GetEquipGroup()
    let eg = card::get_equip_group(d, c);
    // for tc in aux.Next(eg) do if dg:IsContains(tc) then a=a+1 end end
    for tc in group::iter(d, &eg) {
        if dg.contains(tc) {
            a += 1;
        }
    }
    // return #dg-a>=1
    dg.len() as i64 - a >= 1
}
/// `s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsCanBeEffectTarget(e)
    card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==1 then
        if effect::get_label(d, a.e) == 1 {
            // e:SetLabel(0)
            effect::set_label(d, a.e, &[0]);
            // local rg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil)
            let rg = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, 0, NO_EX);
            // local dg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler(),e)
            let dg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, a.e)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
            // return rg:IsExists(s.costfilter,1,nil,rg,dg)
            return group::is_exists(d, &rg, &|d, c| costfilter(d, c, &rg, &dg), 1, NO_EX);
        } else {
            // return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler())
            return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
        }
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // local rg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil)
        let rg = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, 0, NO_EX);
        // local dg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler(),e)
        let dg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, a.e)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local sg1=rg:FilterSelect(tp,s.costfilter,1,1,nil,rg,dg)
        let mut sg1 = group::filter_select(d, &rg, a.tp, &|d, c| costfilter(d, c, &rg, &dg), 1, 1, false, NO_EX).nil();
        // local sc=sg1:GetFirst()
        let sc = group::get_first(d, &sg1).nil();
        // local a=0
        let mut aa: i64 = 0;
        // if dg:IsContains(sc) then a=1 end
        if dg.contains(sc) {
            aa = 1;
        }
        // if sc:GetEquipCount()>0 then
        if card::get_equip_count(d, sc) > 0 {
            // local eqg=sc:GetEquipGroup()
            let eqg = card::get_equip_group(d, sc);
            // for tc in aux.Next(eqg) do if dg:IsContains(tc) then a=a+1 end end
            for tc in group::iter(d, &eqg) {
                if dg.contains(tc) {
                    aa += 1;
                }
            }
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local sg2=rg:FilterSelect(tp,s.costfilter2,1,1,sc,a,dg)
        let sg2 = group::filter_select(d, &rg, a.tp, &|d, c| costfilter2(d, c, aa, &dg), 1, 1, false, sc.into()).nil();
        // sg1:Merge(sg2)
        group::add_card(&mut sg1, &sg2);
        // Duel.SendtoGrave(sg1,REASON_COST)
        duel::send_to_grave(d, sg1, REASON_COST, None, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,3,e:GetHandler())
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 3, false, h.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
