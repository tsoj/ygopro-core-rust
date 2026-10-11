//! Yellow Dragon Ninja
//! 黄竜の忍者
use super::prelude::*;

pub const ID: u32 = 90247311;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NINJA, SET_NINJITSU_ART], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| splimit(d, a.e(1))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, se: EffectId) -> bool {
    // return (se:IsMonsterEffect() and se:GetHandler():IsSetCard(SET_NINJA)) or se:GetHandler():IsSetCard(SET_NINJITSU_ART)
    let h = effect::get_handler(d, se).nil();
    (effect::is_monster_effect(d, se) && card::is_set_card(d, h, &[SET_NINJA as u16])) || card::is_set_card(d, h, &[SET_NINJITSU_ART as u16])
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsMonster() and c:IsSetCard(SET_NINJA)) or c:IsSetCard(SET_NINJITSU_ART))
    //     and (c:IsFaceup() or not c:IsOnField()) and c:IsAbleToGraveAsCost()
    ((card::is_monster(d, c) && card::is_set_card(d, c, &[SET_NINJA as u16])) || card::is_set_card(d, c, &[SET_NINJITSU_ART as u16]))
        && (card::is_faceup(d, c) || !card::is_on_field(d, c))
        && card::is_able_to_grave_as_cost(d, c)
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsSpellTrap() and (not e or c:IsCanBeEffectTarget(e))
    card::is_spell_trap(d, c) && e.is_none_or(|e| card::is_can_be_effect_target(d, c, Some(e)))
}
/// `s.costfilter(c,rg,dg)`
fn costfilter(d: &mut Duel, c: CardId, rg: &CardSet, dg: &CardSet) -> bool {
    // if not (c:IsMonster() and c:IsSetCard(SET_NINJA)) then return false end
    if !(card::is_monster(d, c) && card::is_set_card(d, c, &[SET_NINJA as u16])) {
        return false;
    }
    // local a=0
    let mut a: i64 = 0;
    // if dg:IsContains(c) then a=1 end
    if group::is_contains(dg, c) {
        a = 1;
    }
    // if c:GetEquipCount()==0 then return rg:IsExists(s.costfilter2,1,c,a,dg) end
    if card::get_equip_count(d, c) == 0 {
        return group::is_exists(d, rg, &|d, x| costfilter2(d, x, a, dg), 1, c.into());
    }
    // local eg=c:GetEquipGroup()
    let eg = card::get_equip_group(d, c);
    // local tc=eg:GetFirst()
    let _tc = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if dg:IsContains(tc) then a=a+1 end
        if group::is_contains(dg, tc) {
            a += 1;
        }
    }
    // return rg:IsExists(s.costfilter2,1,c,a,dg)
    group::is_exists(d, rg, &|d, x| costfilter2(d, x, a, dg), 1, c.into())
}
/// `s.costfilter2(c,a,dg)`
fn costfilter2(d: &mut Duel, c: CardId, a: i64, dg: &CardSet) -> bool {
    let mut a = a;
    // if dg:IsContains(c) then a=a+1 end
    if group::is_contains(dg, c) {
        a += 1;
    }
    // return c:IsSetCard(SET_NINJITSU_ART) and #dg-a>=1
    card::is_set_card(d, c, &[SET_NINJITSU_ART as u16]) && dg.len() as i64 - a >= 1
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsOnField() and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && filter(d, chkc, None);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==1 then
        if effect::get_label(d, e) == 1 {
            // e:SetLabel(0)
            effect::set_label(d, e, &[0]);
            // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD|LOCATION_HAND,0,nil)
            let rg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, NO_EX);
            // local dg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,e)
            let dg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, Some(e))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
            // return rg:IsExists(s.costfilter,1,nil,rg,dg)
            return group::is_exists(d, &rg, &|d, c| costfilter(d, c, &rg, &dg), 1, NO_EX);
        } else {
            // return Duel.IsExistingTarget(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
            return duel::is_existing_target(d, Some(&|d, c| filter(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
        }
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD|LOCATION_HAND,0,nil)
        let rg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, NO_EX);
        // local dg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,e)
        let dg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, Some(e))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local sg1=rg:FilterSelect(tp,s.costfilter,1,1,nil,rg,dg)
        let mut sg1 = group::filter_select(d, &rg, tp, &|d, c| costfilter(d, c, &rg, &dg), 1, 1, false, NO_EX).nil();
        // local sc=sg1:GetFirst()
        let sc = group::get_first(d, &sg1).nil();
        // local a=0
        let mut a: i64 = 0;
        // if dg:IsContains(sc) then a=1 end
        if group::is_contains(&dg, sc) {
            a = 1;
        }
        // if sc:GetEquipCount()>0 then
        if card::get_equip_count(d, sc) > 0 {
            // local eqg=sc:GetEquipGroup()
            let eqg = card::get_equip_group(d, sc);
            // local tc=eqg:GetFirst()
            let _tc = group::get_first(d, &eqg);
            // for tc in aux.Next(eqg) do
            for tc in group::iter(d, &eqg) {
                // if dg:IsContains(tc) then a=a+1 end
                if group::is_contains(&dg, tc) {
                    a += 1;
                }
            }
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local sg2=rg:FilterSelect(tp,s.costfilter2,1,1,sc,a,dg)
        let sg2 = group::filter_select(d, &rg, tp, &|d, c| costfilter2(d, c, a, &dg), 1, 1, false, sc.into()).nil();
        // sg1:Merge(sg2)
        group::add_card(&mut sg1, &sg2);
        // Duel.SendtoGrave(sg1,REASON_COST)
        duel::send_to_grave(d, sg1, REASON_COST, None, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,2,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 2, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.as_ref().map_or(0, |g| g.len()) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &g, &|d, x| card::is_relate_to_effect(d, x, a.e), NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
